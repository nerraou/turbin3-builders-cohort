use {
    anchor_lang::{
        solana_program::{instruction::Instruction, msg},
        system_program::ID as SYSTEM_PROGRAM_ID,
        AccountDeserialize, InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::Message,
    solana_pubkey::Pubkey,
    solana_signer::Signer,
    solana_transaction::Transaction,
};

fn setup() -> (LiteSVM, Keypair) {
    let program_id = anchor_marketplace_core::id();

    let payer = Keypair::new();

    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../../target/deploy/anchor_marketplace_core.so");

    svm.add_program(program_id.to_bytes(), program_bytes)
        .unwrap();

    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

    (svm, payer)
}

#[test]
fn test_setup() {
    let (svm, payer) = setup();

    let balance = svm.get_balance(&payer.pubkey()).unwrap();

    assert_eq!(balance, 10_000_000_000);
}
