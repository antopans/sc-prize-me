elrond_wasm::imports!();
elrond_wasm::derive_imports!();

/////////////////////////////////////////////////////////////////////
// Types
/////////////////////////////////////////////////////////////////////

// Fee policy
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct FeePolicy<M: ManagedTypeApi> {
    pub fee_amount_egld: BigUint<M>,
    pub sponsor_reward_percent: u8,
}

/////////////////////////////////////////////////////////////////////
// Functions
/////////////////////////////////////////////////////////////////////
#[elrond_wasm::module]
pub trait FeeModule {

    /////////////////////////////////////////////////////////////////////
    // Endpoints
    /////////////////////////////////////////////////////////////////////
    #[endpoint(setFeePol)]
    fn set_fee_policy(&self, fee_amount_egld: BigUint, sponsor_reward_percent: u8) -> SCResult<()> {
        only_owner!(self, "Caller address not allowed");
        require!(sponsor_reward_percent <= 100, "Wrong value for sponsor reward");

        // Save fee policy
        let fee_policy = FeePolicy {
            fee_amount_egld : fee_amount_egld,
            sponsor_reward_percent : sponsor_reward_percent,
        };

        self.fee_policy_mapper().set(&fee_policy); 

        Ok(())
    }

    #[endpoint(claimFees)]
    fn claim_fees(&self) -> SCResult<()> {
        only_owner!(self, "Caller address not allowed");
        require!(self.fee_pool_mapper().get() != BigUint::zero(), "No fees to claim");
        
        // Claim fees and clear the pool
        self.send().direct_egld(&self.blockchain().get_owner_address(), &self.fee_pool_mapper().get(), b"Fees from pool claimed");
        self.fee_pool_mapper().clear();

        Ok(())
    }
    
    /////////////////////////////////////////////////////////////////////
    // Queries
    /////////////////////////////////////////////////////////////////////
    #[view(getFeePol)]
    fn get_fee_policy(&self) -> MultiResult2<BigUint, u8> {        
        let current_fee_policy: FeePolicy<Self::Api> = self.fee_policy_mapper().get();

        return MultiArg2((current_fee_policy.fee_amount_egld, current_fee_policy.sponsor_reward_percent)); 
    }
    
    #[view(getFeePool)]
    fn get_fee_pool(&self) -> BigUint {
               
        // Get the current amount of fees in the pool
        return self.fee_pool_mapper().get(); 
    }

    /////////////////////////////////////////////////////////////////////
    // Mappers
    /////////////////////////////////////////////////////////////////////
    #[storage_mapper("fee_policy")]
    fn fee_policy_mapper(&self) -> SingleValueMapper<FeePolicy<Self::Api>>;

    #[storage_mapper("fee_pool")]
    fn fee_pool_mapper(&self) -> SingleValueMapper<BigUint>;
}