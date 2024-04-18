use anchor_lang::{prelude::*, system_program};
use solana_program::{account_info::AccountInfo,program::invoke,entrypoint::ProgramResult,};
pub mod constant;
pub mod states;
use spl_token::{instruction::transfer_checked};
use std::collections::HashSet;
use std::iter::FromIterator;
use anchor_lang::solana_program::program_memory::sol_memset;
use anchor_spl::token::{self, Burn, Mint as Minter, Token, TokenAccount, Transfer};
use solana_program::program_pack::Pack;

use crate::{constant::*, states::*};




declare_id!("Cvko9yyXvwFe4pqJr8g9iThZMhCaU1qr54zJJAYmqQFv");

#[program]
mod gig_hub_app {
    use super::*;

    pub fn create_master(_ctx: Context<CreateMaster>) -> Result<(), GigHubError> {
        Ok(())
    }

    pub fn init_user(ctx: Context<InitUser>) -> Result<(), GigHubError> {
        if ctx.accounts.authority.key() != ctx.accounts.user_profile.key() {
            return Err(GigHubError::Unauthorized);
        }
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.authority = ctx.accounts.authority.key();
        user_profile.last_contract = 0;
        user_profile.contract_count = 0;

        Ok(())
    }


    pub fn create_contract_giger(
        ctx: Context<CreateContract>,
        title: String,
        description: String,
        to_assign: Pubkey,
        end_date: u64,
        creation_date: u64,
        price: u64,
        assigned_admin_account: SignerAccounts,
    ) -> Result<(), GigHubError> {
        // Validate input parameters
        if title.is_empty() || description.is_empty() || price == 0 {
            return Err(GigHubError::InvalidInput);
        }

        let contract_account = &mut ctx.accounts.contract_account;
        let user_profile = &mut ctx.accounts.user_profile;

        // Validate authority
        if ctx.accounts.authority.key() != contract_account.authority {
            return Err(GigHubError::Unauthorized);
        }

        contract_account.assigned_admin = Some(assigned_admin_account);
        contract_account.to_assign = to_assign;
        ctx.accounts.master.master_contract_count += 1;
        contract_account.assigned_master = ctx.accounts.master.master_contract_count;
        contract_account.idx = user_profile.last_contract;
        contract_account.title = title;
        contract_account.description = description;
        contract_account.end_date = end_date;
        contract_account.creation_date = creation_date;
        contract_account.price = price;
        contract_account.status = "Deployed".to_string();

        contract_account.created_freelancer = Some(SignerAccounts {
            wallet_adress: ctx.accounts.authority.key(),
        });

        user_profile.last_contract = user_profile
            .last_contract
            .checked_add(1)
            .ok_or(GigHubError::InvalidContractCount)?;

        user_profile.contract_count = user_profile
            .contract_count
            .checked_add(1)
            .ok_or(GigHubError::InvalidContractCount)?;

        Ok(())
    }

    
    pub fn take_contract(
        ctx: Context<TakeContract>,
        assigned_master: u8,

    ) -> Result<(), GigHubError> {
        l let contract_account = &mut ctx.accounts.contract_account;

        // Validate authority
        if ctx.accounts.authority.key() != contract_account.authority {
            return Err(GigHubError::Unauthorized);
        }

        contract_account.confermed_assign = Some(SignerAccounts {
            wallet_adress: ctx.accounts.authority.key(),
        });
        contract_account.status = "InProgress".to_string();

        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.authority.to_account_info(),
                    to: contract_account.to_account_info(),
                },
            ),
            ctx.accounts.contract_account.price,
        )?;

        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.authority.to_account_info(),
                    to: ctx.accounts.assigned_admin.to_account_info(),
                },
            ),
            ctx.accounts.contract_account.price / 100 * 7,
        )?;

        Ok(())
    }

    pub fn do_payment(ctx: Context<DoPayment>, assigned_master: u8, who: u8,) -> Result<(), GigHubError> {
        let contract_account = &mut ctx.accounts.contract_account;

        // Validate authority
        if ctx.accounts.authority.key() != contract_account.authority {
            return Err(GigHubError::Unauthorized);
        }
        
        **ctx
            .accounts
            .contract_account
            .to_account_info()
            .try_borrow_mut_lamports()? -= ctx.accounts.contract_account.price;

        if who == 1{
        **ctx
            .accounts
            .freelancer
            .to_account_info()
            .try_borrow_mut_lamports()? += ctx.accounts.contract_account.price;
            println!("Money Succesfully Send to Employer");
        
        }else if who == 2{
        **ctx
            .accounts
            .confermed_assign
            .to_account_info()
            .try_borrow_mut_lamports()? += ctx.accounts.contract_account.price;     
            println!("Money Succesfully Send to Employee");

        }else{
            print!("Error while who get balance");
        }
        

        Ok(())
    }

           
    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64, assigned_master: u8) -> Result <()> {

        let signer = &ctx.accounts.from;
        let from = &ctx.accounts.from_ata;
        let to_account = &ctx.accounts.contract_account_minted;
        let cpi_accounts = Transfer{
            from: from.to_account_info().clone(),
            to: to_account.to_account_info().clone(),
            authority: signer.to_account_info().clone()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_contex = CpiContext::new(cpi_program, cpi_accounts);

        anchor_spl::token::transfer(cpi_contex, amount);

        let cpi_accounts2 = Transfer{
            from: to_account.to_account_info().clone(),
            to: from.to_account_info().clone(),
            authority: signer.to_account_info().clone()
        };
        let cpi_program2 = ctx.accounts.token_program.to_account_info();
        let cpi_contex2 = CpiContext::new(cpi_program2, cpi_accounts2);

        anchor_spl::token::transfer(cpi_contex2, amount/2);


}
