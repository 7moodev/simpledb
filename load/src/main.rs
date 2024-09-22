use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    pubkey::Pubkey,
    system_instruction,
    commitment_config::{CommitmentConfig, CommitmentLevel},
    instruction::{Instruction, AccountMeta},
    system_program,
};
use std::str::FromStr;
use std::borrow::Borrow;


#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct UserInfo {
    pub first_name: String,
    pub last_name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let commitment_config = CommitmentConfig {
        commitment: CommitmentLevel::Finalized,
    };
    let url = "https://api.testnet.solana.com".to_string();
    let rpc = RpcClient::new(url);
    
    // Generate a new Keypair for the new account
    let newkp = Keypair::new();
    
    // Keypair from a fixed byte array (replace with your own secure keypair)
    // Tailored for ease of use in the example
    let byte_array: [u8; 64] = [181,175,159,47,165,172,244,222,228,246,65,62,178,212,113,
                                112,194,222,184,0,129,67,253,245,33,130,43,123,153,179,
                                85,122,16,124,109,213,201,164,
                                121,172,128,39,83,1,135,50,33,171,84,113,75,224,203,137,
                                133,9,161,79,134,156,55,245,
                                70,102];
    let payer = Keypair::from_bytes(&byte_array).unwrap();

    // Program ID for Solana
    let program_id = Pubkey::from_str("AxU4GMMQvSjuiTHFcA6U2HqRyAu1YpxaZ2imPPhnUMCa").unwrap();
    
    // Create user data
    let first_name = "Sam";
    let last_name = "Bankman-Fried";
    let user_data = UserInfo {
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
    };

    // Serialize the user data using Borsh for size calculation
    let mut serialized_data: Vec<u8> = Vec::new();
    user_data.serialize(&mut serialized_data)?;

    // Calculate lamports for rent exemption
    let lamports = rpc.get_minimum_balance_for_rent_exemption(serialized_data.len())?;

    // Create account instruction
    let create_account_instruction = system_instruction::create_account(
        &payer.pubkey(),        // Payer
        &newkp.pubkey(),        // New user account
        lamports,               // Rent-exempt lamports
        serialized_data.len() as u64, // Data length
        &program_id,            // Owner program ID
    );

    // Fetch the latest blockhash for the transaction
    let mut recent_blockhash = rpc.get_latest_blockhash()?;
    
    // Send the transaction to create the program account 
    let transaction1 = Transaction::new_signed_with_payer(
        &[create_account_instruction],        // Instruction to create account
        Some(&payer.pubkey()),                // Payer of the transaction fees
        &[&payer, &newkp],                    // Signing keypair
        recent_blockhash,                     // Blockhash
    );
    
    // Uncomment the next line to actually send the transaction
    let signature = rpc.send_and_confirm_transaction_with_spinner_and_commitment(&transaction1, commitment_config)?;
    println!("Transaction 1 Signature: {}", signature);

    // Create an instruction to send the serialized user data to the program
    let empty_instruction = Instruction::new_with_borsh(
        program_id,
        &user_data,                     // Serialized user data
        vec![
            AccountMeta { pubkey: payer.pubkey(), is_signer: true, is_writable: true },
            AccountMeta { pubkey: newkp.pubkey(), is_signer: true, is_writable: true },
            AccountMeta { pubkey: program_id, is_signer: false, is_writable: false },
            AccountMeta { pubkey: system_program::ID, is_signer: false, is_writable: false },
        ],
    );
    // Send the second transaction with the user data
    recent_blockhash = rpc.get_latest_blockhash()?;
    let transaction2 = Transaction::new_signed_with_payer(
        &[empty_instruction],                  // Instruction with user data
        Some(&payer.pubkey()),                 // Payer of the transaction fees
        &[&payer, &newkp],                     // Signing keypair
        recent_blockhash,                      // Blockhash
    );
    // Uncomment the next line to send the second transaction
    let signature = rpc.send_and_confirm_transaction(&transaction2)?;
    println!("Transaction 2 Signature: {}", signature);

    // Fetch the account data and deserialize it
    match rpc.get_account(&newkp.pubkey()) {
        Ok(account) => {
            println!("Fetching Data On Chain!!!");
            println!("Account address: {:?}", newkp.pubkey());
            println!("Account owner: {:?}", account.owner);
            let user_info: UserInfo = UserInfo::try_from_slice(&account.data.borrow())?;
            println!("First Name: {}", user_info.first_name);
            println!("Last Name: {}", user_info.last_name);
        }
        Err(e) => eprintln!("Failed to fetch account: {:?}", e),
    }
    Ok(())
}

