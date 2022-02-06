#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod onsenswap_factory {

    const MINIMUM_LIQUIDITY: u128 = 1000; // 10 ** 3 

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[derive(Default)]
    #[ink(storage)]
    pub struct OnsenswapFactory {
        factory: AccountId,
        token0: AccountId,
        token1: AccountId,

        reserve0: u128,
        reserve1: u128,
        blockTimestampLast: u32,

        price0CumulativeLast: u128,
        price1CumulativeLast: u128,
        kLast: u128,
    }

    impl OnsenswapFactory {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(
            factory: AccountId,
            token0: AccountId,
            token1: AccountId,
        ) -> Self {
            // message sender
            let caller = Self::env().caller();
            let mut onsenswapFactory: OnsenswapFactory = Default::default();
            onsenswapFactory.factory = caller;
            onsenswapFactory.token0 = token0;
            onsenswapFactory.token1 = token1;

            onsenswapFactory
        }

        #[ink(message)]
        pub fn get_reserves(&self) -> (u128, u128, u32) {
            (self.reserve0, self.reserve1, self.blockTimestampLast)
        }

        #[ink(message)]
        pub fn mint()
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

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut onsenswap_factory = OnsenswapFactory::new(false);
            assert_eq!(onsenswap_factory.get(), false);
            onsenswap_factory.flip();
            assert_eq!(onsenswap_factory.get(), true);
        }
    }
}
