elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::event;

/////////////////////////////////////////////////////////////////////
// Types
/////////////////////////////////////////////////////////////////////


/////////////////////////////////////////////////////////////////////
// Functions
/////////////////////////////////////////////////////////////////////
#[elrond_wasm::module]
pub trait CharityModule:
    event::EventModule {

    /////////////////////////////////////////////////////////////////////
    // Endpoints
    /////////////////////////////////////////////////////////////////////
    #[only_owner]
    #[endpoint(claimDonations)]
    fn claim_donations(&self) -> SCResult<()> {
        require!(self.charity_pool_mapper().get() != BigUint::zero(), "No donation to claim");

        let donations_amount: BigUint = self.charity_pool_mapper().get();
        
        // Claim donations and clear the pool
        self.send().direct_egld(&self.blockchain().get_owner_address(), &donations_amount, b"Donations from pool claimed");
        self.charity_pool_mapper().clear();

        // Log event
        self.event_wrapper_claim_donations(&donations_amount);

        Ok(())
    }
    
    /////////////////////////////////////////////////////////////////////
    // Queries
    /////////////////////////////////////////////////////////////////////    
    #[view(getCharityPool)]
    fn get_charity_pool(&self) -> BigUint {
               
        // Get the current amount of donations in the pool
        return self.charity_pool_mapper().get(); 
    }

    /////////////////////////////////////////////////////////////////////
    // Internal SC functions
    /////////////////////////////////////////////////////////////////////
    fn init_donations_if_empty(&self) {
        self.charity_pool_mapper().set_if_empty(&BigUint::zero());
    }

    /////////////////////////////////////////////////////////////////////
    // Mappers
    /////////////////////////////////////////////////////////////////////
    
    // Charity pool 
    #[storage_mapper("charity_pool")]
    fn charity_pool_mapper(&self) -> SingleValueMapper<BigUint>;
}