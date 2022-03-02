elrond_wasm::imports!();

use super::instance;
use super::event;

use instance::InstanceStatus;

////////////////////////////////////////////////////////////////////
// Functions
////////////////////////////////////////////////////////////////////
#[elrond_wasm::module]
pub trait SponsorModule: 
    instance::InstanceModule
    +event::EventModule {

    /////////////////////////////////////////////////////////////////////
    // Endpoints
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    // Queries
    /////////////////////////////////////////////////////////////////////
    #[view(getSponsorIDs)]
    fn get_sponsor_instances(&self, sponsor_address: ManagedAddress) -> VarArgs<u32> {
        let mut sponsor_iids = VarArgs::new();

        // Return all instances IDs with sponsor address matching the one provided in parameter
        for (iid, instance_info) in self.instance_info_mapper().iter() {
            if instance_info.sponsor_info.address.clone() == sponsor_address {
                sponsor_iids.push(iid);
            }
        }

        return sponsor_iids;
    }

    #[view(getNbSponsorRunning)]
    fn get_nb_sponsor_running(&self, sponsor_address: ManagedAddress) -> u32 {
        let mut nb_instances: u32 = 0;

        // Compute number of running instances for a specific sponsor
        for (iid, instance_info) in self.instance_info_mapper().iter() {
           
            if instance_info.sponsor_info.address.clone() == sponsor_address && self.get_instance_status(iid) == InstanceStatus::Running {
                nb_instances += 1;
            }
        }

        return nb_instances;
    }
    
    /////////////////////////////////////////////////////////////////////
    // Internal SC functions
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    // Mappers
    /////////////////////////////////////////////////////////////////////
    
    // Number of instances with status 'Running' (or 'Ended') for one sponsor
    #[storage_mapper("nb_instances_running")]
    fn nb_instances_running_mapper(&self, sponsor_address: ManagedAddress) -> SingleValueMapper<u32>;
    
}