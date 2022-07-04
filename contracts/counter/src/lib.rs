//! This contract implements simple counter backed by storage on blockchain.
//!
//! The contract provides methods to [increment] / [decrement] counter and
//! get it's current value [get_num].
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    val: i8,
}

#[near_bindgen]
impl Counter {
    /// Public method: Returns the counter value.
    pub fn get_num(&self) -> i8 {
        self.val
    }

    /// Public method: Increment the counter.
    pub fn increment(&mut self) -> i8 {
        self.val += 1;
        log!("Increased number to {}", self.val);
        self.val
    }

    /// Public method: Decrement the counter.
    pub fn decrement(&mut self) -> i8 {
        self.val -= 1;
        log!("Decreased number to {}", self.val);
        self.val
    }
}

/*
 * The rest of this file sets up unit tests
 * Run with `cargo test`
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment() {
        // instantiate a contract variable with the counter at zero
        let mut contract = Counter { val: 0 };
        contract.increment();
        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn decrement() {
        let mut contract = Counter { val: 0 };
        contract.decrement();
        assert_eq!(-1, contract.get_num());
    }
}
