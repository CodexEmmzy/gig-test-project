use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserProfile{
    pub authority: Pubkey,
    pub last_contract: u8,
    pub contract_count: u8,
}

#[account]
#[derive(Default)]
pub struct ContractAccount {
    pub idx: u8,
    pub authority: Pubkey,
    pub toAssign: Pubkey,
    pub title: String,
    pub description: String,
    pub price: u64,
    pub assigned_master: u8,
    pub created_freelancer: Option<SignerAccounts>,
    pub confermed_assign: Option<SignerAccounts>,
    pub assigned_admin: Option<SignerAccounts>,
    pub status: String,
    pub creation_date: u64,
    pub end_date: u64,
}


#[account]
#[derive(Default)]
pub struct Master{
    pub master_contract_count: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SignerAccounts{
    pub wallet_adress: Pubkey,
}

// Define an enum for error handling
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GigHubError {
    InvalidInput,
    TransferFailed,
    InsufficientFunds,
    Unauthorized
    // Add more error cases as needed
}

impl From<GigHubError> for ProgramError {
    fn from(e: GigHubError) -> Self {
        ProgramError::Custom(e as u32)
    }
}


#[derive(Accounts)]
pub struct CreateMaster<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 8,
        seeds = [b"counter"],
        bump,
    )]
    pub master: Account<'info, Master>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 32 + 1 + 1 + 8,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct CreateContract<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        seeds = [b"counter"],
        bump,
    )]
    pub master: Account<'info, Master>,

    #[account(
        init,
        seeds = [CONTRACT_TAG, &(master.master_contract_count + 1).to_le_bytes()],
        bump,
        payer = authority,
        space = 1 + 32 + 4 + 256 + 4 + 256 + 4 + 32 + 4 + 50 + 8,
    )]
    pub contract_account: Box<Account<'info, ContractAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(assigned_master: u8)]
pub struct TakeContract<'info> {
    #[account(
        mut,
        seeds = [CONTRACT_TAG, &[contract_account.assigned_master],],
        bump,
    )]
    pub contract_account: Box<Account<'info, ContractAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut, address = contract_account.assigned_admin.as_ref().unwrap().wallet_adress)]
    pub assigned_admin: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(assigned_master: u8)]
pub struct DoPayment<'info> {
    #[account(
        mut,
        seeds = [CONTRACT_TAG, &[contract_account.assigned_master]],
        bump,
    )]
    pub contract_account: Box<Account<'info, ContractAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut, address = contract_account.created_freelancer.as_ref().unwrap().wallet_adress)]
    pub freelancer: AccountInfo<'info>,
    #[account(mut, address = contract_account.confermed_assign.as_ref().unwrap().wallet_adress)]
    pub confermed_assign: AccountInfo<'info>,
    #[account(mut, address = contract_account.assigned_admin.as_ref().unwrap().wallet_adress)]
    pub assigned_admin: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(assigned_master: u8)]
pub struct TransferTokens<'info> {

        #[account(
        mut,
        seeds = [CONTRACT_TAG, &[contract_account.assigned_master]],
        bump,
    )]
    pub contract_account: Box<Account<'info, ContractAccount>>,

    #[account(
        init,
        payer = from,
        seeds = [MINT_TAG, CONTRACT_TAG, &[contract_account.assigned_master]],
        token::mint= mint_account_adress,
        token::authority = contract_account,
        bump,
    )]
    pub contract_account_minted: Account<'info, TokenAccount>,

    #[account(mut)]
    pub from: Signer<'info>,
    
    #[account(mut)]
    pub from_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub mint_account_adress: Account<'info, Minter>,
    pub system_program: Program<'info, System>,
}