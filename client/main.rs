use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};

use std::str::FromStr;
use user_management_program::instruction::UserInstruction;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = "http://localhost:8899".to_string();
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Usa tu path al keypair aquí si usas uno existente, o usa Keypair::new() para pruebas
    let payer: Keypair = Keypair::new();
    //let payer = read_keypair_file("~/.config/solana/id.json")?;

    println!("Payer: {}", payer.pubkey());

    // Airdrop para pruebas locales
    client.request_airdrop(&payer.pubkey(), 2_000_000_000)?;

    // Espera hasta que los fondos estén disponibles
    while client.get_balance(&payer.pubkey())? == 0 {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    let user_account = Keypair::new();
    let program_id = Pubkey::from_str("<REEMPLAZA_CON_TU_PROGRAM_ID>")?;

    let instruction_data = UserInstruction::CreateProfile {
        username: "jesusvalencia".to_string(),
        email: "jesus@example.com".to_string(),
    }
    .try_to_vec()?;

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(user_account.pubkey(), false),
            AccountMeta::new(payer.pubkey(), true),
        ],
        data: instruction_data,
    };

    // Crear cuenta de usuario
    let rent = client.get_minimum_balance_for_rent_exemption(1024)?;
    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &user_account.pubkey(),
        rent,
        1024,
        &program_id,
    );

    let blockhash = client.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[create_account_ix, instruction],
        Some(&payer.pubkey()),
        &[&payer, &user_account],
        blockhash,
    );

    let sig = client.send_and_confirm_transaction(&tx)?;
    println!("Transacción enviada: {}", sig);

    Ok(())
}
