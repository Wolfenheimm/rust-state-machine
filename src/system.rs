use core::ops::AddAssign;
use std::collections::BTreeMap;

use num::traits::{One, Zero};

pub trait Config {
    type AccountId: Ord + ToString + Clone;
    type BlockNumber: Zero + One + AddAssign + Copy;
    type Nonce: Zero + One + Copy;
}
#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    // Get the current block number
    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    // Increase the block number
    pub fn inc_block_number(&mut self) {
        self.block_number += T::BlockNumber::one();
    }

    // Increment the nonce
    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let new_nonce: T::Nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero()) + T::Nonce::one();
        self.nonce.insert(who.clone(), new_nonce);
    }

    pub fn get_nonce(&mut self, who: &T::AccountId) -> T::Nonce {
        *self.nonce.get(who).unwrap_or(&T::Nonce::zero())
    }

    // New System Pallet
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Index;

    #[test]
    fn init_system() {
        struct TestConfig;
        impl super::Config for TestConfig {
            type AccountId = String;
            type BlockNumber = u32;
            type Nonce = u32;
        }

        let mut system_pallet = super::Pallet::<TestConfig>::new();
        let alice = "alice";
        let bob = "bob";

        // increment the current block number
        system_pallet.inc_block_number();

        // Instantiate alice then increment the nonce of alice
        system_pallet.nonce.insert(alice.to_string(), 0);
        system_pallet.inc_nonce(&alice.to_string());

        // Instantiate bob
        system_pallet.nonce.insert(bob.to_string(), 0);

        assert_eq!(system_pallet.block_number, 1);
        assert_eq!(*system_pallet.nonce.index(alice), 1);
        assert_eq!(*system_pallet.nonce.index(bob), 0);
    }
}
