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
        account_to_score: storage::HashMap<AccountId, u32>,

        /// Store AccountIds on the leaderboard in storage
        accounts: storage::Vec<AccountId>,
    }

    /// Events
    /// See https://substrate.dev/substrate-contracts-workshop/#/2/creating-an-event
    #[ink(event)]
    struct SetAccountScore {
        #[ink(topic)]
        of: AccountId,
        #[ink(topic)]
        score: u32
    }

    // TODO - restore once ink! 3.0 is released
    // /// Errors that can occur upon calling this contract.
    // /// Reference: https://github.com/paritytech/ink/blob/master/examples/dns/lib.rs
    // #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    // #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    // pub enum Error {
    //     /// Returned if caller is not owner while required to.
    //     CallerIsNotOwner,
    // }

    // TODO - restore once ink! 3.0 is released
    // /// Type alias for the contract's result type.
    // /// Reference: https://github.com/paritytech/ink/blob/master/examples/dns/lib.rs
    // pub type Result<T> = core::result::Result<T, Error>;
    pub type Result<T, U> = core::result::Result<T, U>;

    impl Leaderboard {
        /// Constructor

        #[ink(constructor)]
        fn new(&mut self) {
            // IMPORTANT: Initialize all storage values
            // See https://substrate.dev/substrate-contracts-workshop/#/1/storing-a-value?id=initializing-storage
            self.owner.set(self.env().caller());
            let initialising_account = AccountId::from([0x1; 32]);
            self.accounts.push(initialising_account);
            self.account_to_score.insert(AccountId::from([0x1; 32]), 0);
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

        // // Get all scores for the all AccountIds
        // #[ink(message)]
        // fn get_all_scores(&self) -> Result<Vec<storage::HashMap<AccountId, u32>>, &'static str> {
        //     let mut all_account_to_scores: Vec<storage::HashMap<AccountId, u32>> = Vec::new();
            
        //     let mut score: u32;
        //     let mut account_scores: storage::HashMap<AccountId, u32>;
        //     for account in self.accounts.iter().cloned() {
        //         let score = self.account_to_score.get(&account);
        //         match score {
        //             None => Err("Error: Unable to find score for account"),
        //             Some(x) => {
        //                 account_scores.insert(
        //                     account,
        //                     *score.unwrap_or(&0),
        //                 );
        //                 all_account_to_scores.push(account_scores);
        //                 Ok(&all_account_to_scores)
        //             },
        //         };
        //     }
        //     Ok(all_account_to_scores)
        // }


        // Set the score for a given AccountId
        #[ink(message)]
        fn set_score_of_account(&mut self, of: AccountId, score: u32) -> Result<(), &'static str> {
            let owner = self.get_owner();
            if of != owner {
                // return Err(Error::CallerIsNotOwner)
                return Err("Error: CallerIsNotOwner")
            }
            match self.account_to_score.get(&of) {
                Some(_) => {
                    self.account_to_score.mutate_with(&of, |value| *value = score);
                }
                None => {
                    self.account_to_score.insert(of, score);
                    self.accounts.push(of);
                }
            };

            // Emit event
            self.env()
                .emit_event(
                    SetAccountScore {
                        of,
                        score,
                    });

            Ok(())
        }

        // Set the score for the calling AccountId
        #[ink(message)]
        fn set_score_of_sender(&mut self, score: u32) -> Result<(), &'static str> {
            let caller = self.env().caller();
            let owner = self.get_owner();
            if caller != owner {
                // return Err(Error::CallerIsNotOwner)
                return Err("Error: CallerIsNotOwner")
            }
            match self.account_to_score.get(&caller) {
                Some(_) => {
                    self.account_to_score.mutate_with(&caller, |value| *value = score);
                }
                None => {
                    self.account_to_score.insert(caller, score);
                    self.accounts.push(caller);
                }
            };

            // Emit event
            self.env()
                .emit_event(
                    SetAccountScore {
                        of: caller,
                        score,
                    });
            
            Ok(())
        }

        /// Returns the contract owner.
        /// Reference: https://github.com/paritytech/ink/blob/master/examples/dns/lib.rs
        #[ink(message)]
        fn get_owner(&self) -> AccountId {
            *self.owner.get()
        }

        /// Private functions

        /// Returns the score for an AccountId or 0 if it is not set.
        fn account_score_or_zero(&self, of: &AccountId) -> u32 {
            let score = self.account_to_score.get(of).unwrap_or(&0);
            *score
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        // Free Functions

        /// Returns a dummy AccountId for unit tests
        fn test_get_dummy_account() -> AccountId {
            [0u8; 32].into()
        }

        fn test_get_owner() -> AccountId {
            [1u8; 32].into()
        }

        #[test]
        fn get_score_of_account_works() {
            let leaderboard = Leaderboard::new();
            assert_eq!(leaderboard.get_score_of_account(test_get_dummy_account()), 0);
        }

        #[test]
        fn get_score_of_sender_works() {
            let leaderboard = Leaderboard::new();
            assert_eq!(leaderboard.get_score_of_sender(), 0);
        }

        #[test]
        fn get_owner_works() {
            let leaderboard = Leaderboard::new();
            assert_eq!(leaderboard.get_owner(), test_get_owner());
        }

        #[test]
        fn set_score_of_sender_works() {
            let mut leaderboard = Leaderboard::new();
            assert_eq!(leaderboard.set_score_of_sender(1), Ok(()));
            assert_eq!(leaderboard.get_score_of_sender(), 1);
        }

        #[test]
        fn set_score_of_account_is_ok_when_sender_equals_given_account_works() {
            let mut leaderboard = Leaderboard::new();
            assert_eq!(leaderboard.get_owner(), test_get_owner());
            assert_eq!(leaderboard.set_score_of_account(test_get_owner(), 2), Ok(()));
            assert_eq!(leaderboard.get_score_of_account(test_get_owner()), 2);
            assert_eq!(leaderboard.get_score_of_account(test_get_dummy_account()), 0);
        }

        #[test]
        fn set_score_of_account_errors_when_sender_not_equal_given_account_works() {
            let mut leaderboard = Leaderboard::new();
            assert_eq!(leaderboard.get_owner(), test_get_owner());
            assert_eq!(leaderboard.set_score_of_account(test_get_dummy_account(), 2), Err("Error: CallerIsNotOwner"));
            assert_eq!(leaderboard.get_score_of_account(test_get_dummy_account()), 0);
            assert_eq!(leaderboard.get_score_of_account(test_get_owner()), 0);
        }

        // TODO - Add tests for Events
        // See: https://paritytech.github.io/ink/ink_core/env/test/fn.recorded_events.html
        // Pending this issue to export the necessary EmittedEvent type from ink_core
        // https://github.com/paritytech/ink/issues/468
    }
}
