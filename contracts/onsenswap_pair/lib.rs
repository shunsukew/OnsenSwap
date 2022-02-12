#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
mod onsenswap_pair {
    use ink_prelude::{
        string::String,
        vec::Vec,
    };
    use onsenswap_project::traits::psp22_token::*;
    use brush::{
        contracts::{
            ownable::*,
            psp22::PSP22Error,
        },
        modifiers,
    };

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum OnsenswapError {
        Custom(String),
        PSP22Error(PSP22Error)
    }

    impl From<OwnableError> for OnsenswapError {
        fn from(ownable: OwnableError) -> Self {
            match ownable {
                OwnableError::CallerIsNotOwner => OnsenswapError::Custom(String::from("O::CallerIsNotOwner")),
                OwnableError::NewOwnerIsZero => OnsenswapError::Custom(String::from("O::NewOwnerIsZero")),
            }
        }
    }

    #[derive(Default, OwnableStorage)]
    #[ink(storage)]
    pub struct OnsenswapPair {
        #[OwnableStorageField]
        ownable: OwnableData,

        factory: AccountId,
        token0_account_id: AccountId,
        token1_account_id: AccountId,

        reserve0: Balance,
        reserve1: Balance,
        blocknumber_last: u32,

        k_last: u128,
    }

    impl Ownable for OnsenswapPair {}

    impl OnsenswapPair {
        #[ink(constructor)]
        pub fn new(token0_account_id: AccountId, token1_account_id: AccountId) -> Self {
            // message sender
            let caller = Self::env().caller();
            let mut onsenswap_pair: OnsenswapPair = Default::default();
            onsenswap_pair.factory = caller;
            onsenswap_pair.token0_account_id = token0_account_id;
            onsenswap_pair.token1_account_id = token1_account_id;
            
            onsenswap_pair._init_with_owner(caller);

            onsenswap_pair
        }

        #[ink(message)]
        pub fn get_reserves(&self) -> (Balance, Balance, u32) {
            (self.reserve0, self.reserve1, self.blocknumber_last)
        }

        #[ink(message)]
        pub fn getk_last(&self) -> u128 {
            self.k_last
        }

        #[ink(message)]
        pub fn swap(&mut self, amount0_out: Balance, amount1_out: Balance, to: AccountId, data: Vec<u8>) -> Result<(), OnsenswapError> {
            if !(0 < amount0_out || 0 < amount1_out) {
                return Err(OnsenswapError::Custom(String::from("OnsenswapV1: Insufficent output amount")))
            }

            let (current_reserve0, current_reserve1, _) = self.get_reserves();

            if !(amount0_out < current_reserve0 && amount1_out < current_reserve1) {
                return Err(OnsenswapError::Custom(String::from("OnsenswapV1: Insufficent liquidity")))
            }

            let contract_address = Self::env().account_id();

            if to == self.token0_account_id || to == self.token1_account_id {
                return Err(OnsenswapError::Custom(String::from("OnsenswapV1: INVALID_TO")))
            }

            // transfer token0 amount0 to to_address
            if let Err(e) = PSP22TokenRef::transfer_from(&self.token0_account_id, contract_address, to, amount0_out, data.clone()) {
                return Err(OnsenswapError::PSP22Error(e))
            }
            // transfer token1 amount1 to to_address
            if let Err(e) = PSP22TokenRef::transfer_from(&self.token1_account_id, contract_address, to, amount1_out, data.clone()) {
                return Err(OnsenswapError::PSP22Error(e))
            }

            // set balance0 value
            let balance0 = PSP22TokenRef::balance_of(&self.token0_account_id, contract_address);
            // set balance1 value
            let balance1 = PSP22TokenRef::balance_of(&self.token1_account_id, contract_address);

            let amount0_in: Balance = if balance0 > current_reserve0 - amount0_out {balance0 - (current_reserve0 - amount0_out)} else {0};
            let amount1_in: Balance = if balance1 > current_reserve1 - amount1_out {balance1 - (current_reserve1 - amount1_out)} else {0};


            // ensure K
            let balance0_adjusted = balance0 * 1000 - amount0_in * 3;
            let balance1_adjusted = balance1 * 1000 - amount1_in * 3;
            if !(balance0_adjusted * balance1_adjusted >= current_reserve0 * current_reserve1 * (1000*1000)) {
                return Err(OnsenswapError::Custom(String::from("OnsenswapV1: K")))
            }

            self.update(balance0, balance1);

            Ok(())
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn skim(&mut self, to: AccountId) -> Result<(), OnsenswapError> {
            let contract_address = Self::env().account_id();
            let balance0 = PSP22TokenRef::balance_of(&self.token0_account_id, contract_address);
            let balance1 = PSP22TokenRef::balance_of(&self.token1_account_id, contract_address);
            if let Err(e) = PSP22TokenRef::transfer_from(&self.token0_account_id, contract_address, to, balance0 - self.reserve0, Vec::<u8>::new()) {
                return Err(OnsenswapError::PSP22Error(e))
            }
            // transfer token1 amount1 to to_address
            if let Err(e) = PSP22TokenRef::transfer_from(&self.token1_account_id, contract_address, to, balance1 - self.reserve1, Vec::<u8>::new()) {
                return Err(OnsenswapError::PSP22Error(e))
            }

            Ok(())
        }

        // update reserves and, on the first call per block, price accumulators
        fn update(&mut self, new_balance0: Balance, new_balance1: Balance) {
            let blocknumber: BlockNumber = Self::env().block_number();
            if blocknumber != self.blocknumber_last && self.reserve0 != 0 && self.reserve1 != 0 {
                unimplemented!();
            }

            self.reserve0 = new_balance0;
            self.reserve1 = new_balance1;
            self.blocknumber_last = blocknumber;
        }
    }
}
