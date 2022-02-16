elrond_wasm::imports!();

use super::instance;
use super::event;

////////////////////////////////////////////////////////////////////
// Functions
////////////////////////////////////////////////////////////////////
#[elrond_wasm::module]
pub trait PlayerModule: 
    instance::InstanceModule
    +event::EventModule {

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
    // Internal SC functions
    /////////////////////////////////////////////////////////////////////
    fn add_player(&self, iid: u32, player_address: &ManagedAddress) -> usize {
        self.instance_players_set_mapper(iid).insert(player_address.clone());
        self.instance_players_vec_mapper(iid).push(player_address);

        // Return number of players as a ticket number
        return self.get_nb_players(iid);
    }

    fn get_nb_players(&self, iid: u32) -> usize {

        // Return number of players
        return self.instance_players_vec_mapper(iid).len();
    }

    fn get_ticket_owner(&self, iid: u32, ticket_number:usize) -> ManagedAddress {

        // Return ticket owner
        return self.instance_players_vec_mapper(iid).get(ticket_number);
    }

    fn clear_players(&self, iid: u32) {
        self.instance_players_set_mapper(iid).clear();
        self.instance_players_vec_mapper(iid).clear();
    }

    /////////////////////////////////////////////////////////////////////
    // Mappers
    /////////////////////////////////////////////////////////////////////
    #[storage_mapper("instance_players_set")]
    fn instance_players_set_mapper(&self, iid: u32) -> SetMapper<ManagedAddress>;

    #[storage_mapper("instance_players_vec")]
    fn instance_players_vec_mapper(&self, iid: u32) -> VecMapper<ManagedAddress>;
    
}