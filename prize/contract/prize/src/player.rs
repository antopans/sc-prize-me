elrond_wasm::imports!();

use super::instance;

////////////////////////////////////////////////////////////////////
// Functions
////////////////////////////////////////////////////////////////////
#[elrond_wasm::module]
pub trait PlayerModule: 
    instance::InstanceModule {

    /////////////////////////////////////////////////////////////////////
    // Endpoints
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    // Queries
    /////////////////////////////////////////////////////////////////////
    #[view(getPlayerIDs)]
    fn get_player_instances(&self, player_address: ManagedAddress) -> VarArgs<u32> {
        let mut player_iids = VarArgs::new();

        // Return all instances IDs to which player address provided in parameter has played
        for iid in self.instance_info_mapper().keys() {
            if self.has_played(iid, player_address.clone()) == true {
                player_iids.push(iid.clone());
            }
        }

        return player_iids;
    }
    
    #[view(hasPlayed)]
    fn has_played(&self, iid: u32, player_address: ManagedAddress) -> bool {
        // Return true is player_address provided in parameter is part of the SetMapper for the specified instance ID
        return self.instance_players_set_mapper(iid).contains(&player_address);
    }

    /////////////////////////////////////////////////////////////////////
    // Mappers
    /////////////////////////////////////////////////////////////////////
    #[storage_mapper("instance_players_set")]
    fn instance_players_set_mapper(&self, iid: u32) -> SetMapper<ManagedAddress>;

    #[storage_mapper("instance_players_vec")]
    fn instance_players_vec_mapper(&self, iid: u32) -> VecMapper<ManagedAddress>;
}