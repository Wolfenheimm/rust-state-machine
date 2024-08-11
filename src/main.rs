use crate::support::Dispatch;

mod balances;
mod system;
mod support;
mod proof_of_existence;

mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = crate::support::Header<BlockNumber>;
    pub type Block = crate::support::Block<Header, Extrinsic>;
    pub type Content = &'static str;
}

#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
    proof_of_existence: proof_of_existence::Pallet<Self>,
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
    type Content = types::Content;
}

fn main() {
    let mut runtime = Runtime::new();
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice, 100);

    runtime.system.inc_nonce(&alice);
    let _res = runtime
        .balances
        .transfer(alice.clone(), bob.clone(), 30)
        .map_err(|e| eprintln!("{}", e));

    runtime.system.inc_nonce(&alice);
    let _res = runtime
        .balances
        .transfer(alice.clone(), charlie.clone(), 20)
        .map_err(|e| eprintln!("{}", e));

    runtime.balances.set_balance(&alice, 100);

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer { to: bob.clone(), amount: 69 }),
            },
        ],
    };

    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "Hello, Alice Created a Claim!"
                }),
            },
            support::Extrinsic {
                caller: bob.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "Hello, Bob Created a Claim!"
                }),
            },
        ],
    };

    let block_3 = types::Block {
        header: support::Header { block_number: 3 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim {
                    claim: "Hello, Alice Created a Claim!"
                }),
            },
            support::Extrinsic {
                caller: bob.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim {
                    claim: "Hello, Bob Created a Claim!"
                }),
            },
            support::Extrinsic {
                caller: charlie.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "Hello, Alice and Bob aren't here right now... Can I help you?!"
                }),
            },
        ],
    };

    runtime.execute_block(block_1).expect("invalid block");
    println!("{:#?}", runtime); // Print runtime information

    runtime.execute_block(block_2).expect("invalid block");
    println!("{:#?}", runtime); // Print runtime information

    runtime.execute_block(block_3).expect("invalid block");
    println!("{:#?}", runtime); // Print runtime information
}
