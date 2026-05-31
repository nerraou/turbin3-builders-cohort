// use {
//     anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL, litesvm::LiteSVM,
//     solana_keypair::Keypair, solana_message::Address, solana_signer::Signer,
// };

// fn setup() -> (LiteSVM, Keypair) {
//     let payer = Keypair::new();

//     let mut svm = LiteSVM::new();

//     // Your program
//     let staking_bytes = include_bytes!("../../../target/deploy/anchor_core_staking.so");

//     svm.add_program(anchor_core_staking::id().to_bytes(), staking_bytes);

//     // MPL Core
//     let mpl_core_bytes = include_bytes!(
//         "../../../target/sbpf-solana-solana/release/deps/mpl_core-7b2e92fcf0c7c23a.so"
//     );
//     svm.add_program(Address::from(mpl_core::ID.to_bytes()), mpl_core_bytes)
//         .unwrap();

//     svm.airdrop(&payer.pubkey(), 10 * LAMPORTS_PER_SOL).unwrap();

//     (svm, payer)
// }
