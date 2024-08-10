use std::collections::BTreeMap;

use num::traits::{CheckedAdd, CheckedSub, Zero};

pub trait Config: crate::system::Config {
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    // storage mapping from accounts (String) to balances (u128)
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    // Set the balance of an account
    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

    // Get the balance of an account
    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

    // Transfers an amount from one account to another, if possible
    pub fn transfer(
        &mut self,
        caller: &T::AccountId,
        to: &T::AccountId,
        amount: T::Balance,
    ) -> crate::support::DispatchResult {
        // Get the balances of caller and to
        let from_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        // Use safe math to calculate a new_from_balance & new_to_balance
        let new_from_balance = from_balance
            .checked_sub(&amount)
            .ok_or("Not enough funds")?;
        let new_to_balance = to_balance.checked_add(&amount).ok_or("Possible overflow")?;

        // Set the new balances for both accounts
        self.set_balance(caller, new_from_balance);
        self.set_balance(to, new_to_balance);

        Ok(())
    }

    // New Balances Pallet
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }
}

// Describes the calls we want to expose to the dispatcher
// Note: The dispatcher handles the caller of each call and thus not included here
pub enum Call<T: Config> {
    Transfer { to: T::AccountId, amount: T::Balance },
}

// Implementation of dispatch logic
impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;

    fn dispatch(
        &mut self,
        caller: Self::Caller,
        call: Self::Call,
    ) -> crate::support::DispatchResult {
        match call {
            Call::Transfer { to, amount } => {
                self.transfer(&caller, &to, amount)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    struct TestConfig;

    impl crate::system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    impl super::Config for TestConfig {
        type Balance = u32;
    }

    #[test]
    fn init_balances() {
        let mut balances = super::Pallet::<TestConfig>::new();

        assert_eq!(balances.balance(&"alice".to_string()), 0);
        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.balance(&"alice".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
        balances
            .transfer(&"alice".to_string(), &"bob".to_string(), 100)
            .unwrap();
        assert_eq!(balances.balance(&"alice".to_string()), 0);
        assert_eq!(balances.balance(&"bob".to_string()), 100)
    }
}
