use ink_lang as ink;

#[ink::contract]
mod flipper {
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

    impl Flipper {
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
            })
        }

        /// Constructor that initializes the `bool` value to `false`.
        #[ink(constructor)]
        pub fn default() -> Self {
            // This call is required in order to correctly initialize the
            // `Mapping`s of our contract.
            ink_lang::utils::initialize_contract(|_| {})
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) -> Result<(),()>{
            
            let caller:AccountId = Self::env().caller();
            let num:u32 = self.caller_to_number.get(caller).unwrap_or_default();

            self.caller = caller;
            self.value = !self.value;
            self.caller_to_number.insert(caller, &(&num+1));
            
            Ok(())
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
        /// Simply returns the current value stored in caller
        #[ink(message)]
        pub fn get_caller(&self) -> AccountId {
            self.caller
        }

        /// Simple function to access mapping
        #[ink(message)]
        pub fn get_caller_value(&self,caller_id:AccountId) -> u32 {
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

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let flipper = Flipper::default();
            assert_eq!(flipper.get(), false);
        }

        /// We test a simple use case of our contract.
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
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            let count = flipper.get_caller_value(accounts.alice);
            let caller = flipper.get_caller();
            assert_eq!(count, 2);
            assert_eq!(caller, accounts.alice);
            assert_eq!(flipper.get(), true);

        }
    }
}
