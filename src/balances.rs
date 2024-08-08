use std::collections::BTreeMap;
pub struct Pallet {
    // storage mapping from accounts (String) to balances (u128)
    balances: BTreeMap<String, u128>
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new()
        }
    }
}