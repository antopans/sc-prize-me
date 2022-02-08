elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::event;

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
pub trait FeeModule:
    event::EventModule {

    /////////////////////////////////////////////////////////////////////
    // Endpoints
    /////////////////////////////////////////////////////////////////////
    #[endpoint(setFeePol)]
    fn set_fee_policy(&self, fee_amount_egld: BigUint, sponsor_reward_percent: u8) -> SCResult<()> {
        only_owner!(self, "Caller address not allowed");
        require!(sponsor_reward_percent <= 100, "Wrong value for sponsor reward");

        // Save fee policy
        let fee_policy = FeePolicy {
            fee_amount_egld : fee_amount_egld.clone(),
            sponsor_reward_percent : sponsor_reward_percent.clone(),
        };

        self.fee_policy_mapper().set(&fee_policy); 

        // Log event
        self.event_wrapper_set_fee_policy(&fee_amount_egld, sponsor_reward_percent);

        Ok(())
    }

    #[endpoint(claimFees)]
    fn claim_fees(&self) -> SCResult<()> {
        only_owner!(self, "Caller address not allowed");
        require!(self.fee_pool_mapper().get() != BigUint::zero(), "No fees to claim");

        let fee_amount: BigUint = self.fee_pool_mapper().get();
        
        // Claim fees and clear the pool
        self.send().direct_egld(&self.blockchain().get_owner_address(), &fee_amount, b"Fees from pool claimed");
        self.fee_pool_mapper().clear();

        // Log event
        self.event_wrapper_claim_fees(&fee_amount);

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
    // Internal SC functions
    /////////////////////////////////////////////////////////////////////
    fn init_fees_if_empty(&self, fee_amount_egld: BigUint, sponsor_reward_percent: u8) {
        self.fee_pool_mapper().set_if_empty(&BigUint::zero());

        self.fee_policy_mapper().set_if_empty(&FeePolicy {
            fee_amount_egld : fee_amount_egld,
            sponsor_reward_percent : sponsor_reward_percent,
        });
    }

    fn update_fees_and_compute_rewards(&self, fees: BigUint, reward_percent: BigUint) -> BigUint {
        let mut reward_amount: BigUint = BigUint::zero();

        // Capitalize fees and compute sponsor rewards
        if fees != BigUint::zero() {

            // Compute sponsor rewards
            reward_amount = fees.clone() * reward_percent.clone() / BigUint::from(100u8);
            let remaining_fees: BigUint = fees.clone() - reward_amount.clone();

            // Add fees to pool
            self.fee_pool_mapper().update(|current_fees| *current_fees += remaining_fees.clone());

            // Log event
            self.event_wrapper_fee_pool_info(&self.fee_pool_mapper().get()); 
        }

        // Return computed rewards
        return reward_amount;
    }

    /////////////////////////////////////////////////////////////////////
    // Mappers
    /////////////////////////////////////////////////////////////////////
    
    // Fee policy 
    #[storage_mapper("fee_policy")]
    fn fee_policy_mapper(&self) -> SingleValueMapper<FeePolicy<Self::Api>>;

    // Fee  pool for SC owner
    #[storage_mapper("fee_pool")]
    fn fee_pool_mapper(&self) -> SingleValueMapper<BigUint>;
}