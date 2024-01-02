use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh0_10::try_from_slice_unchecked,
    bpf_loader_upgradeable::close_any,
    clock::UnixTimestamp,
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_program,
    sysvar::{clock::Clock, rent, Sysvar},
};
use solana_sdk::{
    signature::{read_keypair_file, Keypair, Signer},
    system_instruction::create_account,
    transaction::Transaction,
};
use spl_associated_token_account::{create_associated_token_account, get_associated_token_address};
use spl_token::{
    instruction::{initialize_mint, mint_to},
    state::{Account, Mint},
};
use std::env;
use std::str::FromStr;
use token_factory::{instruction::*, state::*, utils::*};

const PROGRAM_ID: &str = "CJjdBvJv6mC7czvpr5d7vZ6oAmmMKuAL48q7ebbfsx2M";
const METADATA_PROGRAM: &str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";

fn config_dev(mint_pubkey: &Pubkey) {
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    dotenv().ok();
    let kp_str = env::var("SECRET").unwrap();
    let signer = Keypair::from_base58_string(&kp_str.as_str());
    let token_program = spl_token::ID;
    let auth = signer.pubkey();
    let signer_pubkey = signer.pubkey();
    let seeds = &[program_id.as_ref(), "config".as_bytes(), "1".as_bytes()];
    let (config_info, _) = Pubkey::find_program_address(seeds, &program_id);
    println!(" config_info::::::{:?}", config_info.to_string());

    let seeds = &[program_id.as_ref(), "round".as_bytes()];
    let (round_info, _) = Pubkey::find_program_address(seeds, &program_id);
    println!(" round_info::::::{:?}", round_info.to_string());

    let seeds = &[
        program_id.as_ref(),
        mint_pubkey.as_ref(),
        "mint_vault".as_bytes(),
    ];
    let (mint_vault, _) = Pubkey::find_program_address(seeds, &program_id);
    println!(" mint_vault::::::{:?}", mint_vault.to_string());

    let seeds = &[
        program_id.as_ref(),
        mint_pubkey.as_ref(),
        "transfer_auth".as_bytes(),
    ];
    let (transfer_auth, _) = Pubkey::find_program_address(seeds, &program_id);
    println!(" transfer_auth::::::{:?}", transfer_auth.to_string());

    let mut instructions = vec![];
    let configargs = ConfigureArgs {

        authority: auth,
        charge_addr: auth,
        round: 1.to_string(),
        start_time: 1704181500,
        total_reward: 100000000000000,
    };

    instructions.push(
        configure(
            &program_id,
            &signer_pubkey,
            &config_info,
            &round_info,
            mint_pubkey,
            &mint_vault,
            &transfer_auth,
            configargs
        )
        .unwrap(),
    );

    let mut transaction = Transaction::new_with_payer(&instructions, Some(&signer.pubkey()));
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let mut signers = vec![&signer];
    signers.push(&signer);
    transaction.sign(&signers, recent_blockhash);
    let signature = client.send_and_confirm_transaction(&transaction).unwrap();

    println!("signature:::{:?}", &signature);
}

fn buy_dev() {
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    dotenv().ok();
    let kp_str = env::var("SECRET").unwrap();
    let signer = Keypair::from_base58_string(&kp_str.as_str());
    let token_program = spl_token::ID;
    let auth = signer.pubkey();
    let signer_pubkey = signer.pubkey();
    let seeds = &[program_id.as_ref(), "config".as_bytes(), "1".as_bytes()];
    let (config_info, _) = Pubkey::find_program_address(seeds, &program_id);
    println!(" config_info::::::{:?}", config_info.to_string());

    let seeds = &[program_id.as_ref(), signer_pubkey.as_ref(), "user_info".as_bytes(), "1".as_bytes()];
    let (user_info, _) = Pubkey::find_program_address(seeds, &program_id);
    println!(" user_info::::::{:?}", user_info.to_string());

    let mut instructions = vec![];
    let buyargs = BuyTicketsArgs {

        shot: Some([1,2,3,4,5,6]),
        num:5
    };

    instructions.push(
        buy(
            &program_id,
            &signer_pubkey,
            &config_info,
            &user_info,
            &signer_pubkey,
            buyargs
        )
        .unwrap(),
    );

    let mut transaction = Transaction::new_with_payer(&instructions, Some(&signer.pubkey()));
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let mut signers = vec![&signer];
    signers.push(&signer);
    transaction.sign(&signers, recent_blockhash);
    let signature = client.send_and_confirm_transaction(&transaction).unwrap();

    println!("signature:::{:?}", &signature);
}

fn close_dev() {
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    dotenv().ok();
    let kp_str = env::var("SECRET").unwrap();
    let signer = Keypair::from_base58_string(&kp_str.as_str());
    let token_program = spl_token::ID;
    let auth = signer.pubkey();
    let signer_pubkey = signer.pubkey();

    let seeds = &[program_id.as_ref(), "round".as_bytes()];
    let (round_info, _) = Pubkey::find_program_address(seeds, &program_id);
    println!(" round_info::::::{:?}", round_info.to_string());

    let seeds = &[program_id.as_ref(), "config".as_bytes(), "1".as_bytes()];
    let (config_info, _) = Pubkey::find_program_address(seeds, &program_id);
    println!(" config_info::::::{:?}", config_info.to_string());

    let seeds = &[program_id.as_ref(), "config".as_bytes(), "2".as_bytes()];
    let (new_config_info, _) = Pubkey::find_program_address(seeds, &program_id);
    println!(" new_config_info::::::{:?}", new_config_info.to_string());


    let mut instructions = vec![];
    
    instructions.push(
        close(
            &program_id,
            &signer_pubkey,
            &config_info,
            &round_info,
            &new_config_info,
        )
        .unwrap(),
    );

    let mut transaction = Transaction::new_with_payer(&instructions, Some(&signer.pubkey()));
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let mut signers = vec![&signer];
    signers.push(&signer);
    transaction.sign(&signers, recent_blockhash);
    let signature = client.send_and_confirm_transaction(&transaction).unwrap();

    println!("signature:::{:?}", &signature);
}

fn claim_dev(mint_pubkey: &Pubkey) {
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    dotenv().ok();
    let kp_str = env::var("SECRET").unwrap();
    let signer = Keypair::from_base58_string(&kp_str.as_str());
    let token_program = spl_token::ID;
    let auth = signer.pubkey();
    let signer_pubkey = signer.pubkey();
    let seeds = &[program_id.as_ref(), "config".as_bytes(), "1".as_bytes()];
    let (config_info, _) = Pubkey::find_program_address(seeds, &program_id);
    println!(" config_info::::::{:?}", config_info.to_string());

    let claimargs = ClaimArgs {
        round:1
    };
    let round = claimargs.round.to_string();
    let seeds = &[program_id.as_ref(), signer_pubkey.as_ref(), "user_info".as_bytes(), round.as_bytes()];
    let (user_info, _) = Pubkey::find_program_address(seeds, &program_id);
    println!(" user_info::::::{:?}", user_info.to_string());

    let seeds = &[
        program_id.as_ref(),
        mint_pubkey.as_ref(),
        "mint_vault".as_bytes(),
    ];
    let (mint_vault, _) = Pubkey::find_program_address(seeds, &program_id);
    println!(" mint_vault::::::{:?}", mint_vault.to_string());

    let seeds = &[
        program_id.as_ref(),
        mint_pubkey.as_ref(),
        "transfer_auth".as_bytes(),
    ];
    let (transfer_auth, _) = Pubkey::find_program_address(seeds, &program_id);
    println!(" transfer_auth::::::{:?}", transfer_auth.to_string());
    let mut instructions = vec![];

    let token_account = get_associated_token_address(&signer_pubkey, &mint_pubkey);
    println!("nft_token_account:::{:?}", token_account);

    if client.get_balance(&token_account).unwrap() == 0 {
        let new_token_account_instruction =
            create_associated_token_account(&signer_pubkey, &signer_pubkey, &mint_pubkey);
        instructions.push(new_token_account_instruction);
    }
    instructions.push(
        claim(
            &program_id,
            &signer_pubkey,
            &config_info,
            &mint_pubkey,
            &user_info,
            &mint_vault,
            &transfer_auth,
            &token_account,
            claimargs
        )
        .unwrap(),
    );

    let mut transaction = Transaction::new_with_payer(&instructions, Some(&signer.pubkey()));
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let mut signers = vec![&signer];
    signers.push(&signer);
    transaction.sign(&signers, recent_blockhash);
    let signature = client.send_and_confirm_transaction(&transaction).unwrap();

    println!("signature:::{:?}", &signature);
}

fn clear_dev(mint_pubkey: &Pubkey) {
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    dotenv().ok();
    let kp_str = env::var("SECRET").unwrap();
    let signer = Keypair::from_base58_string(&kp_str.as_str());
    let token_program = spl_token::ID;
    let auth = signer.pubkey();
    let signer_pubkey = signer.pubkey();
    let seeds = &[program_id.as_ref(), "config".as_bytes(), "1".as_bytes()];
    let (config_info, _) = Pubkey::find_program_address(seeds, &program_id);
    println!(" config_info::::::{:?}", config_info.to_string());

    let clearargs = ClearArgs {
        amt:1
    };

    let seeds = &[
        program_id.as_ref(),
        mint_pubkey.as_ref(),
        "mint_vault".as_bytes(),
    ];
    let (mint_vault, _) = Pubkey::find_program_address(seeds, &program_id);
    println!(" mint_vault::::::{:?}", mint_vault.to_string());

    let seeds = &[
        program_id.as_ref(),
        mint_pubkey.as_ref(),
        "transfer_auth".as_bytes(),
    ];
    let (transfer_auth, _) = Pubkey::find_program_address(seeds, &program_id);
    println!(" transfer_auth::::::{:?}", transfer_auth.to_string());
    let mut instructions = vec![];

    let token_account = get_associated_token_address(&signer_pubkey, &mint_pubkey);
    println!("nft_token_account:::{:?}", token_account);

    if client.get_balance(&token_account).unwrap() == 0 {
        let new_token_account_instruction =
            create_associated_token_account(&signer_pubkey, &signer_pubkey, &mint_pubkey);
        instructions.push(new_token_account_instruction);
    }
    instructions.push(
        clear(
            &program_id,
            &signer_pubkey,
            &config_info,
            &mint_pubkey,
            &mint_vault,
            &transfer_auth,
            &token_account,
            clearargs
        )
        .unwrap(),
    );

    let mut transaction = Transaction::new_with_payer(&instructions, Some(&signer.pubkey()));
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let mut signers = vec![&signer];
    signers.push(&signer);
    transaction.sign(&signers, recent_blockhash);
    let signature = client.send_and_confirm_transaction(&transaction).unwrap();

    println!("signature:::{:?}", &signature);
}


fn main() {

    // let mint_pubkey = Pubkey::from_str("BNMjgfzampFZ2JL1qMBnQ8oZG5vwDUaXYkwJLWmrSJ6u").unwrap();
    // config_dev(&mint_pubkey);

    let round_info = Pubkey::from_str("8NLFvdo6ocuueUsXvs1Q4S8NzxRRm5Ge36JoFAVhdPVy").unwrap();
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let account = client.get_account(&round_info).unwrap();
    let roundata: RoundData = try_from_slice_unchecked(&account.data).unwrap();
    println!("roundata:::{:?}", roundata);

    let config_info = Pubkey::from_str("BSQw1nQSWhKKm3LBJ8GAT3kuv118D1oWdEgD8M321u5w").unwrap();
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let account = client.get_account(&config_info).unwrap();
    let configdata: ConfigureData = try_from_slice_unchecked(&account.data).unwrap();
    println!("configdata:::{:?}", configdata);
    

}
