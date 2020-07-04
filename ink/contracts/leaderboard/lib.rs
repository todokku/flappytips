#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
mod leaderboard {
    use ink_core::storage;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    struct Leaderboard {
        /// Store a contract owner
        owner: storage::Value<AccountId>,

        //// Store a mapping from AccountIds to a u32 of user on the leaderboard in the storage
        account_map: storage::HashMap<AccountId, u32>,
    }

    impl Leaderboard {
        /// Constructor

        #[ink(constructor)]
        fn new(&mut self) {
            // IMPORTANT: Initialize all storage values
            // See https://substrate.dev/substrate-contracts-workshop/#/1/storing-a-value?id=initializing-storage
            self.owner.set(self.env().caller());

            self.account_map.insert(AccountId::from([0x1; 32]), 0);
        }

        /// Public Functions

        // Get the score for a given AccountId
        #[ink(message)]
        fn get_score_of_account(&self, of: AccountId) -> u32 {
            let value = self.account_score_or_zero(&of);
            value
        }

        // Get the score for the calling AccountId
        #[ink(message)]
        fn get_score_of_sender(&self) -> u32 {
            let caller = self.env().caller();
            let value = self.account_score_or_zero(&caller);
            value
        }

        // Set the score for the calling AccountId
        #[ink(message)]
        fn set_score_of_sender(&self, score: u32) -> () {
            let caller = self.env().caller();
            match self.account_map.get(&caller) {
                Some(_) => {
                    self.account_map.mutate_with(&caller, |value| *value += score);
                }
                None => {
                    self.account_map.insert(caller, score);
                }
            };
        }

        // Set the score for a given AccountId
        #[ink(message)]
        fn set_score_of_account(&self, of: AccountId, score: u32) -> () {
            let caller = self.env().caller();
            match self.account_map.get(&of) {
                Some(_) => {
                    self.account_map.mutate_with(&of, |value| *value += score);
                }
                None => {
                    self.account_map.insert(of, score);
                }
            };
        }

        /// Private functions

        /// Returns the score for an AccountId or 0 if it is not set.
        fn account_score_or_zero(&self, of: &AccountId) -> u32 {
            let score = self.account_map.get(of).unwrap_or(&0);
            *score
        }
    }

    // Free Functions

    /// Returns a dummy AccountId for unit tests
    fn get_dummy_account() -> AccountId {
        let account: AccountId = [0u8; 32].into();
        account
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        #[test]
        fn constructor_new_works() {
            let leaderboard = Leaderboard::new();
        }

        #[test]
        fn get_score_of_account_works() {
            let mut leaderboard = Leaderboard::new();
            assert_eq!(leaderboard.get_score_of_account(get_dummy_account()), 0);
        }

        #[test]
        fn get_score_of_sender_works() {
            let mut leaderboard = Leaderboard::new();
            assert_eq!(leaderboard.get_score_of_sender(), 0);
        }
    }
}