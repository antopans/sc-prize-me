elrond_wasm::imports!();

use super::instance;
use super::instance::InstanceStatus;

use super::event;

////////////////////////////////////////////////////////////////////
// Functions
////////////////////////////////////////////////////////////////////
#[elrond_wasm::module]
pub trait SecurityModule:
    instance::InstanceModule
    +event::EventModule {

    /////////////////////////////////////////////////////////////////////
    // Endpoints
    /////////////////////////////////////////////////////////////////////
    #[only_owner]
    #[endpoint(disable)]
    fn disable_instance(&self, iid: u32, disable_status: bool) -> SCResult<()> {
        require!(self.get_instance_status(iid) != InstanceStatus::NotExisting, "Instance does not exist");
        
        // Retrieve instance state
        let mut instance_state = self.instance_state_mapper().get(&iid).unwrap();
        
        if instance_state.disabled != disable_status {
            instance_state.disabled = disable_status;
            self.instance_state_mapper().insert(iid, instance_state);

            // Log event
            self.event_wrapper_disable_instance(iid, disable_status);

            Ok(())
        }
        else{
            sc_error!("Instance is already in the expected disable state")
        }
    } 

    #[only_owner]
    #[endpoint(addAddrBlacklist)]
    fn add_addr_blacklist(&self, address: ManagedAddress) -> SCResult<()> {        
        if self.address_blacklist_set_mapper().insert(address.clone()) == true {

            // Log event
            self.event_wrapper_add_addr_blacklist(&address);

            Ok(())
        }
        else {
            sc_error!("Address already blacklisted")
        }
    }

    #[only_owner]
    #[endpoint(rmAddrBlacklist)]
    fn rm_addr_blacklist(&self, address: ManagedAddress) -> SCResult<()> {        
        if self.address_blacklist_set_mapper().remove(&address) == true {
            
            // Log event
            self.event_wrapper_rm_addr_blacklist(&address);

            Ok(())
        }
        else {
            sc_error!("Address not blacklisted")
        }
    }
    
    /////////////////////////////////////////////////////////////////////
    // Queries
    /////////////////////////////////////////////////////////////////////  
    #[view(getAddrBlacklist)]
    fn get_address_blacklist(&self) ->  VarArgs<ManagedAddress>  {
               
        let mut address_blacklist: VarArgs<ManagedAddress> = VarArgs::new();

        for addr in self.address_blacklist_set_mapper().iter() {
            address_blacklist.push(addr);
        }

        return address_blacklist;
    }

    /////////////////////////////////////////////////////////////////////
    // Mappers
    /////////////////////////////////////////////////////////////////////
    #[storage_mapper("address_blacklist_set")]
    fn address_blacklist_set_mapper(&self) -> SetMapper<ManagedAddress>;
}