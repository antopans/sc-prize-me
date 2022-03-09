elrond_wasm::imports!();

use super::event;

////////////////////////////////////////////////////////////////////
// Functions
////////////////////////////////////////////////////////////////////
#[elrond_wasm::module]
pub trait ParameterModule:
    event::EventModule {

    /////////////////////////////////////////////////////////////////////
    // Endpoints
    /////////////////////////////////////////////////////////////////////
    #[only_owner]
    #[endpoint(setParamManClaim)]
    fn set_param_manual_claim(&self, manual_claim: bool) -> SCResult<()> {    
        self.param_manual_claim_mapper().update(|current_value| *current_value = manual_claim);

        // Log event
        self.event_wrapper_set_param_manual_claim(manual_claim);

        Ok(())
    }

    #[only_owner]
    #[endpoint(setParamNbMaxInstancesPerSponsor)]
    fn set_param_nb_max_instances_per_sponsor(&self, nb_instances_max: u32) -> SCResult<()> {        
        self.param_nb_max_instances_per_sponsor_mapper().update(|current_value| *current_value = nb_instances_max);

        // Log event
        self.event_wrapper_set_param_nb_max_instances_per_sponsor(nb_instances_max);

        Ok(())
    }

    #[only_owner]
    #[endpoint(setParamDuration)]
    fn set_param_duration(&self, duration_min: u64, duration_max: u64) -> SCResult<()> {
        require!(duration_min <= duration_max, "Min duration must be lower or equal to Max duration");
        
        self.param_duration_min_mapper().update(|current_value| *current_value = duration_min);
        self.param_duration_max_mapper().update(|current_value| *current_value = duration_max);

        // Log event
        self.event_wrapper_set_param_duration(duration_min, duration_max);

        Ok(())
    }

    /////////////////////////////////////////////////////////////////////
    // Queries
    /////////////////////////////////////////////////////////////////////
    #[view(getParamManClaim)]
    fn get_param_manual_claim(&self) -> bool {        
        return self.param_manual_claim_mapper().get(); 
    }

    #[view(getParamNbMaxInstancesPerSponsor)]
    fn get_param_nb_max_instances_per_sponsor(&self) -> u32 {        
        return self.param_nb_max_instances_per_sponsor_mapper().get(); 
    }

    #[view(getParamDuration)]
    fn get_param_duration(&self) -> MultiValue2<u64, u64> {   
        return MultiValue2((self.param_duration_min_mapper().get(), self.param_duration_max_mapper().get()));     
    }

    /////////////////////////////////////////////////////////////////////
    // Mappers
    /////////////////////////////////////////////////////////////////////
    #[storage_mapper("param_manual_claim")]
    fn param_manual_claim_mapper(&self) -> SingleValueMapper<bool>;

    #[storage_mapper("param_nb_max_instances_per_sponsor")]
    fn param_nb_max_instances_per_sponsor_mapper(&self) -> SingleValueMapper<u32>;

    #[storage_mapper("param_duration_min")]
    fn param_duration_min_mapper(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("param_duration_max")]
    fn param_duration_max_mapper(&self) -> SingleValueMapper<u64>;
}