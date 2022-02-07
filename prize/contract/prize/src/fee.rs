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

    fn init_rewards(&self, iid: u32) {
        // Initialize sponsor rewards @ instance creation
        self.reward_percent_mapper(iid).set(self.fee_policy_mapper().get().sponsor_reward_percent);
        self.reward_pool_mapper(iid).set(BigUint::zero());
    }

    fn append_fees_and_rewards(&self, iid: u32, fees: BigUint) {
        // Capitalize fees and set sponsor rewards
        if fees != BigUint::zero() {
            let reward_percent = BigUint::from(self.reward_percent_mapper(iid).get());
            let mut reward_pool = self.reward_pool_mapper(iid).get();

            // Compute sponsor rewards
            let reward_amount: BigUint = fees.clone() * reward_percent.clone() / BigUint::from(100u8);
            let remaining_fees: BigUint = fees.clone() - reward_amount.clone();

            // Add rewards to sponsor rewards pool
            reward_pool += reward_amount.clone();
            self.reward_pool_mapper(iid).set(reward_pool);

            // Add fees to pool
            self.fee_pool_mapper().update(|current_fees| *current_fees += remaining_fees.clone());

            // Log event
            self.event_wrapper_append_fees_and_rewards(iid, &fees, &reward_percent, &reward_amount, &remaining_fees); 
        }
    }

    fn pay_rewards_to_sponsor(&self, iid: u32, sponsor_address: ManagedAddress) {
        // Send rewards to sponsor
        let reward_pool = self.reward_pool_mapper(iid).get();

        if reward_pool > BigUint::zero() {
            self.send().direct_egld(
                &sponsor_address,
                &reward_pool,
                b"Sponsor rewards",
            );
            
            // Log event
            self.event_wrapper_send_rewards(iid, &reward_pool);

            // Clear mappers for this instance
            self.reward_pool_mapper(iid).clear();
            self.reward_percent_mapper(iid).clear();
        }
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

    // Rewards for sponsor
    #[storage_mapper("reward_percent")]
    fn reward_percent_mapper(&self, iid: u32) -> SingleValueMapper<u8>;

    #[storage_mapper("reward_pool")]
    fn reward_pool_mapper(&self, iid: u32) -> SingleValueMapper<BigUint>;
}