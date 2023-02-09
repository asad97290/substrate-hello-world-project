use ink_lang as ink;
use ink_env::AccountId;

/// Interface
#[ink::trait_definition]
pub trait IFlipper {
    #[ink(message)]
    fn flip(&mut self) -> Result<(),()>;
    #[ink(message)]
    fn get(&self) -> bool;
    #[ink(message)]
    fn get_caller(&self) -> AccountId;
    #[ink(message)]
    fn get_caller_value(&self,caller_id:AccountId) -> u32;
}

/// Module Flipper
#[ink::contract]
pub mod flipper {
    use super::IFlipper;
    use ink_storage::traits::SpreadAllocate;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Flipper {
        value: bool,
        caller: AccountId,
        caller_to_number: ink_storage::Mapping<AccountId, u32>,
    }
    
    /// Event emitted when an approval occurs that `spender` is allowed to withdraw
    /// up to the amount of `value` tokens from `owner`.
    #[ink(event)]
    pub struct Flipped {
        #[ink(topic)]
        caller: AccountId,
        #[ink(topic)]
        value:bool,
        #[ink(topic)]
        no_of_times:u32
    }

    /// Flipper Struct Implementation
    impl Flipper{
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            // This call is required in order to correctly initialize the
            // `Mapping`s of our contract.
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                let caller:AccountId = Self::env().caller();
                let num:u32 = 0;
                contract.value = init_value;
                contract.caller = caller;
                contract.caller_to_number.insert(&caller, &(&num+1));
                // emit event
                Self::env().emit_event(Flipped {
                    caller: caller,
                    value: init_value,
                    no_of_times:num
                });
            })
        }

        /// Default Constructor that initializes the `bool` value to `false`.
        #[ink(constructor)]
        pub fn default() -> Self {
            // This call is required in order to correctly initialize the
            // `Mapping`s of our contract.
            ink_lang::utils::initialize_contract(|_| {})
        }
    }

    /// IFlipper trait Implementation
    impl IFlipper for Flipper {

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        fn flip(&mut self) -> Result<(),()>{
            
            let caller:AccountId = Self::env().caller();
            let num:u32 = self.caller_to_number.get(caller).unwrap_or_default();

            self.caller = caller;
            self.value = !self.value;
            self.caller_to_number.insert(caller, &(&num+1));
            // emit event
            Self::env().emit_event(Flipped {
                caller: caller,
                value: self.value,
                no_of_times:num
            });
            Ok(())
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        fn get(&self) -> bool {
            self.value
        }
        /// Simply returns the current value stored in caller
        #[ink(message)]
        fn get_caller(&self) -> AccountId {
            self.caller
        }

        /// Simple function to access mapping
        #[ink(message)]
        fn get_caller_value(&self,caller_id:AccountId) -> u32 {
            self.caller_to_number.get(caller_id).unwrap_or_default()
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            let flipper = Flipper::default();
            assert_eq!(flipper.get(), false);
        }

        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);
            let res = flipper.flip();
            assert_eq!(res, Result::Ok(()));
            assert_eq!(flipper.get(), true);
        }

        #[ink::test]
        fn mapping_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);
           
            let res = flipper.flip();
            assert_eq!(res, Result::Ok(()));
           
            let accounts = get_accounts();
            let count = flipper.get_caller_value(accounts.alice);
            let caller = flipper.get_caller();
            assert_eq!(count, 2);
            assert_eq!(caller, accounts.alice);
            assert_eq!(flipper.get(), true);

        }

        #[ink::test]
        fn change_account_works() {
            let accounts = get_accounts();
            set_caller(accounts.eve);

            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);

            let res = flipper.flip();
            assert_eq!(res, Result::Ok(()));
            
            let count = flipper.get_caller_value(accounts.eve);
            let caller = flipper.get_caller();
            assert_eq!(count, 2);
            assert_eq!(caller, accounts.eve);
            assert_eq!(flipper.get(), true);

        }
        // utility functions
        fn set_caller(sender: AccountId) {
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(sender);
        }

        fn get_accounts() -> ink_env::test::DefaultAccounts<Environment>{
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            return accounts;
        }
    }
}
