use std::collections::BTreeMap;
pub struct Pallet {
    // storage mapping from accounts (String) to balances (u128)
    balances: BTreeMap<String, u128>
}

impl Pallet {
    // Set the balance of an account
    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    // Get the balance of an account
    pub fn balance(&self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    // Transfers an amount from one account to another, if possible
    pub fn transfer(
        &mut self,
        caller: String,
        to: String,
        amount: u128,
    ) -> Result<(), &'static str> {
        // Get the balances of caller and to
        let from_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        // Use safe math to calculate a new_from_balance & new_to_balance
        let new_from_balance = from_balance
            .checked_sub(amount)
            .ok_or("Not enough funds")?;
        let new_to_balance = to_balance
            .checked_add(amount)
            .ok_or("Possible overflow")?;

        // Set the new balances for both accounts
        self.set_balance(&caller, new_from_balance);
        self.set_balance(&to, new_to_balance);

        Ok(())
    }

    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn init_balances() {
        let mut balances = super::Pallet::new();

        assert_eq!(balances.balance(&"alice".to_string()), 0);
        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.balance(&"alice".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
        balances.transfer("alice".to_string(), "bob".to_string(), 100).unwrap();
        assert_eq!(balances.balance(&"alice".to_string()), 0);
        assert_eq!(balances.balance(&"bob".to_string()), 100)
    }
}