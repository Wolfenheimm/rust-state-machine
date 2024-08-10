use crate::support::Dispatch;

mod balances;
mod system;
mod support;

mod types {
    use crate::RuntimeCall;

    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = crate::support::Extrinsic<AccountId, RuntimeCall>;
    pub type Header = crate::support::Header<BlockNumber>;
    pub type Block = crate::support::Block<Header, Extrinsic>;
}

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
pub enum RuntimeCall { // Outer enum
    /* TODO:
        - Ask what this enum variant is, how it works
        - Another kind is found in balances.rs, inside enum Call<T: Config>
    */
    Balances(balances::Call<Runtime>) // Inter enum
}

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl crate::support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;
    // Dispatch a call on behalf of a caller. Increments the caller's nonce.
    //
    // Dispatch allows us to identify which underlying module call we want to execute.
    // Note that we extract the `caller` from the extrinsic, and use that information
    // to determine who we are executing the call on behalf of.
    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> support::DispatchResult {
        match runtime_call {
            RuntimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
            }
        }

        Ok(())
    }
}


impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }

    // Execute a block of extrinsics. Increments the block number.
    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        if self.system.block_number() != block.header.block_number {
            return Err("Incoming block number does not coincide with system block number");
        }

        for (i, support::Extrinsic { call, caller }) in block.extrinsics.into_iter().enumerate() {
            self.system.inc_nonce(&caller);
            let _res = self.dispatch(caller, call).map_err(|e| {
                eprintln!(
                    "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
                    block.header.block_number, i, e
                )
            });
        }

        Ok(())
    }
}

fn main() {
    let mut runtime = Runtime::new();
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice, 100);

    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 1);

    runtime.system.inc_nonce(&alice);
    let _res = runtime
        .balances
        .transfer(&alice, &bob, 30)
        .map_err(|e| eprintln!("{}", e));

    runtime.system.inc_nonce(&alice);
    let _res = runtime
        .balances
        .transfer(&alice, &charlie, 20)
        .map_err(|e| eprintln!("{}", e));

    runtime.balances.set_balance(&alice, 100);

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice,
                call: RuntimeCall::Balances(balances::Call::Transfer { to: bob, amount: 69 }),
            },
        ],
    };
    runtime.execute_block(block_1).expect("invalid block");

    // Print runtime information
    println!("{:#?}", runtime);
}
