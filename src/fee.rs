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
    pub link_reward_percent: u8,
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
    #[only_owner]
    #[endpoint(setFeePol)]
    fn set_fee_policy(&self, fee_amount_egld: BigUint, sponsor_reward_percent: u8, link_reward_percent: u8) -> SCResult<()> {
        require!((sponsor_reward_percent + link_reward_percent) <= 100, "Wrong value for rewards");

        // Save fee policy
        let fee_policy = FeePolicy {
            fee_amount_egld : fee_amount_egld.clone(),
            sponsor_reward_percent : sponsor_reward_percent.clone(),
            link_reward_percent : link_reward_percent.clone(),
        };

        self.fee_policy_mapper().set(&fee_policy); 

        // Log event
        self.event_wrapper_set_fee_policy(&fee_amount_egld, sponsor_reward_percent, link_reward_percent);

        Ok(())
    }

    #[only_owner]
    #[endpoint(claimFees)]
    fn claim_fees(&self) -> SCResult<()> {
        let fee_amount: BigUint = self.fee_pool_mapper().get();

        require!(fee_amount != BigUint::zero(), "No fees to claim");

        // Claim fees and clear the pool
        self.send().direct_egld(&self.blockchain().get_owner_address(), &fee_amount, b"Fees from pool claimed");
        self.fee_pool_mapper().clear();

        // Log event
        self.event_wrapper_claim_fees(&fee_amount);

        Ok(())
    }

    #[endpoint(claimLinkRewards)]
    fn claim_link_rewards(&self) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        let reward_amount: BigUint = self.link_reward_pool_mapper(&caller).get();

        require!(reward_amount != BigUint::zero(), "No rewards to claim");
       
        // Claim rewards and clear the pool
        self.send().direct_egld(&caller, &reward_amount, b"Link rewards claimed");
        self.link_reward_pool_mapper(&caller).clear();

        // Log event
        self.event_wrapper_claim_link_rewards(&reward_amount, &caller);

        Ok(())
    }
    
    /////////////////////////////////////////////////////////////////////
    // Queries
    /////////////////////////////////////////////////////////////////////
    #[view(getFeePol)]
    fn get_fee_policy(&self) -> MultiValue3<BigUint, u8, u8> {        
        let current_fee_policy: FeePolicy<Self::Api> = self.fee_policy_mapper().get();

        return MultiValue3((current_fee_policy.fee_amount_egld, current_fee_policy.sponsor_reward_percent, current_fee_policy.link_reward_percent)); 
    }
    
    #[view(getFeePool)]
    fn get_fee_pool(&self) -> BigUint {
               
        // Get the current amount of fees in the pool
        return self.fee_pool_mapper().get(); 
    }

    #[view(getLinkRewardPool)]
    fn get_link_reward_pool(&self, link_address: ManagedAddress) -> BigUint {
               
        // Get the current amount of rewards in the pool for the address provided in parameter
        return self.link_reward_pool_mapper(&link_address).get(); 
    }

    /////////////////////////////////////////////////////////////////////
    // Internal SC functions
    /////////////////////////////////////////////////////////////////////
    fn init_fees_if_empty(&self, fee_amount_egld: BigUint, sponsor_reward_percent: u8, link_reward_percent: u8) {
        self.fee_pool_mapper().set_if_empty(&BigUint::zero());

        self.fee_policy_mapper().set_if_empty(&FeePolicy {
            fee_amount_egld : fee_amount_egld,
            sponsor_reward_percent : sponsor_reward_percent,
            link_reward_percent: link_reward_percent
        });
    }

    fn update_fees_and_compute_rewards(&self, fees: BigUint, sponsor_reward_percent: u8, link_address: Option<ManagedAddress>) -> BigUint {
        let mut link_reward_percent: u8 = 0;
        let mut sponsor_reward_amount: BigUint = BigUint::zero();
        let link_reward_amount: BigUint;

        // Capitalize fees and compute sponsor rewards
        if fees != BigUint::zero() {

            // Apply link rewards only if an affiliation link address has been provided
            if link_address.is_some() == true {
                link_reward_percent = self.fee_policy_mapper().get().link_reward_percent;

                // Sponsor reward percent is the value at the lottery creation while Link reward percent is the current value
                // Ensure the sum of rewards does not overflow the fees (100 %); truncate link reward if so
                // This is a safeguard measure, this condition should never be true
                if (sponsor_reward_percent + link_reward_percent) > 100 {
                    link_reward_percent = 100 - sponsor_reward_percent;
                }
            };
            
            // Compute rewards
            sponsor_reward_amount = fees.clone() * BigUint::from(sponsor_reward_percent) / BigUint::from(100u8);
            link_reward_amount = fees.clone() * BigUint::from(link_reward_percent) / BigUint::from(100u8);
            let remaining_fees: BigUint = fees - sponsor_reward_amount.clone() - link_reward_amount.clone();

            // Add fees to pool
            self.fee_pool_mapper().update(|current_fees| *current_fees += remaining_fees);

            //Add link rewards to affiliation pool 
            if link_address.is_some() == true && link_reward_amount > BigUint::zero() {
                self.link_reward_pool_mapper(&link_address.unwrap()).update(|current_link_rewards| *current_link_rewards += link_reward_amount);
            }

            // Log event
            self.event_wrapper_fee_pool_info(&self.fee_pool_mapper().get()); 
        }

        // Return computed sponsor rewards
        return sponsor_reward_amount;
    }

    /////////////////////////////////////////////////////////////////////
    // Mappers
    /////////////////////////////////////////////////////////////////////
    
    // Fee policy 
    #[storage_mapper("fee_policy")]
    fn fee_policy_mapper(&self) -> SingleValueMapper<FeePolicy<Self::Api>>;

    // Fee pool for SC owner
    #[storage_mapper("fee_pool")]
    fn fee_pool_mapper(&self) -> SingleValueMapper<BigUint>;

    // Reward pool for affiliation links (per address)
    #[storage_mapper("link_reward_pool")]
    fn link_reward_pool_mapper(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;
}