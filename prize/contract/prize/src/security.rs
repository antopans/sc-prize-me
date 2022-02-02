elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait SecurityModule {

/////////////////////////////////////////////////////////////////////
    // Endpoints
    /////////////////////////////////////////////////////////////////////
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