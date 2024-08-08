use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet {
    block_number: u32,
    pub(crate) nonce: BTreeMap<String, u32>
}

impl Pallet {

    // Get the current block number
    pub fn block_number(&self) -> u32 {
        self.block_number
    }

    // Increase the block number
    pub fn inc_block_number(&mut self){
        self.block_number += 1;
    }

    // Increment the nonce
    pub fn inc_nonce(&mut self, who: &String){
        let nonce_value = self.nonce.get(who).unwrap_or(&0) + 1;
        self.nonce.insert(who.to_string(), nonce_value);
    }

    // New System Pallet
    pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce: BTreeMap::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Index;

    #[test]
    fn init_system(){
        let mut system_pallet = super::Pallet::new();
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