mod constants;
mod errors;
mod events;
mod instructions;
mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("GrwRJ5fgKwethiy3Zf5rAYuYJVv938FT3N4hiwV71H8K");

#[program]
pub mod dippies_index_protocol {
    use super::*;

    pub fn create_forest(
        ctx: Context<CreateLeaderboard>,
        id: Pubkey,
        tree_creation_fee: u64,
    ) -> Result<()> {
        instructions::create_leaderboard(ctx, id, tree_creation_fee)
    }

    pub fn set_forest(ctx: Context<SetLeaderboardFee>, entry_creation_fee: u64) -> Result<()> {
        instructions::set_leaderboard_fee(ctx, entry_creation_fee)
    }

    pub fn create_entry(ctx: Context<CreateEntry>) -> Result<()> {
        instructions::create_entry(ctx)
    }

    pub fn insert_entry(ctx: Context<InsertEntry>) -> Result<()> {
        instructions::insert_entry(ctx)
    }

    pub fn replace_entry(ctx: Context<ReplaceEntry>) -> Result<()> {
        instructions::replace_entry(ctx)
    }

    pub fn create_stake_deposit(ctx: Context<CreateStakeDeposit>) -> Result<()> {
        instructions::create_stake_deposit(ctx)
    }

    pub fn update_stake(ctx: Context<UpdateStake>, stake: i128) -> Result<()> {
        instructions::update_stake(ctx, stake)
    }

    pub fn set_bribe(ctx: Context<SetBribe>, amount: u64) -> Result<()> {
        instructions::set_bribe(ctx, amount)
    }

    pub fn claim_bribe(ctx: Context<ClaimBribe>) -> Result<()> {
        instructions::claim_bribe(ctx)
    }
}
