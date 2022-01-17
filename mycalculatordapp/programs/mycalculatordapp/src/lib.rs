use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");


// Programming Macros
#[program]
pub mod mycalculatordapp {
    use super::*;

    
    // We want to keep track of the greeting message, result of any computation and remainder for division
    // Calculator account needs to be mutable as the same account has to be used at multiple places
    
    // An account is not actually a wallet but a way for the contract to persist the data between the calls such as
    // count, permissions on account
    // Accounts pay rent in form of lamports,and if lamports => 0 then account is purged from the blockchain

    // Accounts with two years worth of rent attached are “rent-exempt” and can stay on the chain forever.

            // Context : Solana accounts array and program ID
    pub fn create(ctx:Context<Create>, init_message:String) -> ProgramResult {

        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        
        // For Error Handling
        // Sends the program in another state if during the flow Ok(()) faces an error 
        
        Ok(())
    }
    
    //Context<Addition> It is the list of accounts that must be passed to this particular function for it to run.
    pub fn add(ctx:Context<Addition>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 + num2;
        Ok(())
    }

    //Context<Addition> It is the list of accounts that must be passed to this particular function for it to run.
    pub fn multiply(ctx:Context<Multiplication>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1*num2;
        Ok(())
    }
    
    //Context<Addition> It is the list of accounts that must be passed to this particular function for it to run.
    pub fn subtract(ctx:Context<Subtraction>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 - num2;
        Ok(())
    }

    //Context<Addition> It is the list of accounts that must be passed to this particular function for it to run.
    pub fn divide(ctx:Context<Division>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1/num2;
        calculator.remainder = num1 % num2;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Create<'info>{
    // Specifying calculator account used in the other programs
    // Specifying user account that pays the Gas Fees in SOL
    // Specifying system_program which are system specifications for the Solana Blockchain

    // All of them in the form of Accounts

    #[account(init, payer = user, space = 8 + 64 + 64 + 64 + 64)]

    // Whenever init parameter is used, we must always specify the payer or the account that will
    // be paying for creation of the account on the Solana blockchain, along with the space param which is the space
    // with which the new account is created.
    
    pub calculator: Account<'info, Calculator>,
    
    // mut marks the account as mutable
    // and that Solana will need to update the data
    // Hence using mut for persisting changes
    #[account(mut)]

    // Signer type is used to enforce constrainst that authority account has signed the transaction
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Addition<'info>{
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Multiplication<'info>{
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Subtraction<'info>{
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Division<'info>{
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[account]
pub struct Calculator {
    pub greeting: String,
    pub result: i64,
    pub remainder: i64,
}