use crate::*;
use anchor_lang::prelude::{pubkey, AccountMeta, Pubkey};
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::{InstructionData, ToAccountMetas};

pub const MEMO_PROGRAM_ID: Pubkey = pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");

#[allow(clippy::too_many_arguments)]
pub fn swap2_instruction(
    lb_pair_pubkey: Pubkey,
    lb_pair: &LbPair,
    bin_array_bitmap_extension: Option<Pubkey>,
    user: Pubkey,
    user_token_in: Pubkey,
    user_token_out: Pubkey,
    amount_in: u64,
    min_amount_out: u64,
    bin_arrays: &[Pubkey],
    transfer_hook: Option<(Vec<RemainingAccountsSlice>, Vec<AccountMeta>)>,
) -> Result<Instruction> {
    let [token_x_program, token_y_program] = lb_pair.get_token_programs()?;
    let (event_authority, _bump) = derive_event_authority_pda();
    let (slices, transfer_hook_accounts) = transfer_hook.unwrap_or_default();

    let accounts = dlmm::client::accounts::Swap2 {
        lb_pair: lb_pair_pubkey,
        bin_array_bitmap_extension,
        reserve_x: lb_pair.reserve_x,
        reserve_y: lb_pair.reserve_y,
        user_token_in,
        user_token_out,
        token_x_mint: lb_pair.token_x_mint,
        token_y_mint: lb_pair.token_y_mint,
        oracle: lb_pair.oracle,
        host_fee_in: None,
        user,
        token_x_program,
        token_y_program,
        memo_program: MEMO_PROGRAM_ID,
        event_authority,
        program: dlmm::ID,
    }
    .to_account_metas(None)
    .into_iter()
    .chain(transfer_hook_accounts)
    .chain(
        bin_arrays
            .iter()
            .map(|&bin_array| AccountMeta::new(bin_array, false)),
    )
    .collect();

    let data = dlmm::client::args::Swap2 {
        amount_in,
        min_amount_out,
        remaining_accounts_info: RemainingAccountsInfo { slices },
    }
    .data();

    Ok(Instruction {
        program_id: dlmm::ID,
        accounts,
        data,
    })
}
