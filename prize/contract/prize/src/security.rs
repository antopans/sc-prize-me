elrond_wasm::imports!();


use super::instance;
use super::instance::InstanceStatus;

////////////////////////////////////////////////////////////////////
// Functions
////////////////////////////////////////////////////////////////////
#[elrond_wasm::module]
pub trait SecurityModule:
    instance::InstanceModule {

    /////////////////////////////////////////////////////////////////////
    // Endpoints
    /////////////////////////////////////////////////////////////////////
    #[endpoint(disable)]
    fn disable_instance(&self, iid: u32, disable_status: bool) -> SCResult<()> {
        only_owner!(self, "Caller address not allowed");
        require!(self.get_instance_status(iid) != InstanceStatus::NotExisting, "Instance does not exist");
        
        // Retrieve instance state
        let mut instance_state = self.instance_state_mapper().get(&iid).unwrap();
        
        if instance_state.disabled != disable_status {
            instance_state.disabled = disable_status;
            self.instance_state_mapper().insert(iid, instance_state);

            Ok(())
        }
        else{
            sc_error!("Instance is already in the expected disable state")
        }
    } 

    #[endpoint(addAddrBlacklist)]
    fn add_addr_blacklist(&self, address: ManagedAddress) -> SCResult<()> {
        only_owner!(self, "Caller address not allowed");
        
        if self.address_blacklist_set_mapper().insert(address) == true {
            Ok(())
        }
        else {
            sc_error!("Address already blacklisted")
        }
    }

    #[endpoint(rmAddrBlacklist)]
    fn rm_addr_blacklist(&self, address: ManagedAddress) -> SCResult<()> {
        only_owner!(self, "Caller address not allowed");
        
        if self.address_blacklist_set_mapper().remove(&address) == true {
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