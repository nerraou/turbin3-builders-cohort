use {
    anchor_lang::{
        solana_program::{instruction::Instruction, msg},
        system_program::ID as SYSTEM_PROGRAM_ID,
        AccountDeserialize, InstructionData, ToAccountMetas,
    },
    anchor_spl::associated_token,
    litesvm::LiteSVM,
    litesvm_token::CreateMint,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_pubkey::Pubkey,
    solana_signer::{signers, Signer},
    solana_transaction::{versioned::VersionedTransaction, Transaction},
};

fn send(
    svm: &mut LiteSVM,
    ixs: &[Instruction],
    payer: &Keypair,
    signers: &[&Keypair],
) -> litesvm::types::TransactionResult {
    svm.expire_blockhash();
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(ixs, Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), signers).unwrap();
    svm.send_transaction(tx)
}

fn setup() -> (
    LiteSVM,
    Keypair,
    Pubkey,
    Pubkey,
    Pubkey,
    Pubkey,
    Pubkey,
    Pubkey,
) {
    let program_id = anchor_amm::id();
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/anchor_amm.so");
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

    let mint_x = CreateMint::new(&mut svm, &payer)
        .decimals(6)
        .authority(&payer.pubkey())
        .send()
        .unwrap();

    let mint_y = CreateMint::new(&mut svm, &payer)
        .decimals(6)
        .authority(&payer.pubkey())
        .send()
        .unwrap();

    let config =
        Pubkey::find_program_address(&[b"config", &123u64.to_le_bytes()], &anchor_amm::id()).0;

    let mint_lp = Pubkey::find_program_address(&[b"lp", &config.as_ref()], &anchor_amm::id()).0;

    let vault_x = associated_token::get_associated_token_address(&config, &mint_x);
    let vault_y = associated_token::get_associated_token_address(&config, &mint_y);

    (
        svm, payer, mint_x, mint_y, config, mint_lp, vault_x, vault_y,
    )
}

#[test]
fn test_initialize() {
    // let program_id = anchor_amm::id();
    // let payer = Keypair::new();
    // let mut svm = LiteSVM::new();
    // let bytes = include_bytes!("../../../target/deploy/anchor_amm.so");
    // svm.add_program(program_id, bytes).unwrap();
    // svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    // let instruction = Instruction::new_with_bytes(
    //     program_id,
    //     &anchor_amm::instruction::Initialize {}.data(),
    //     anchor_amm::accounts::Initialize {}.to_account_metas(None),
    // );

    // let blockhash = svm.latest_blockhash();
    // let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    // let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer]).unwrap();

    // let res = svm.send_transaction(tx);
    // assert!(res.is_ok());
}
