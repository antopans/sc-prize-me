#![no_std]

elrond_wasm::imports!();

// Modules
mod security;
mod parameter;
mod fee;

// Types
mod common_types;
use common_types::*;
use fee::FeePolicy;

// Macros
mod macros;

#[elrond_wasm::contract]
pub trait Prize: 
    security::SecurityModule 
    +parameter::ParameterModule
    +fee::FeeModule {
    
    /////////////////////////////////////////////////////////////////////
    // SC Management API
    /////////////////////////////////////////////////////////////////////

    #[init]
    fn init(&self) -> SCResult<()> {
        const DEFAULT_MIN_DURATION: u64 = 60;           // 60 seconds
        const DEFAULT_MAX_DURATION: u64 = 60*60*24*365; // 1 year
        
        // Initializations @ deployment only 

        // Instances
        self.iid_counter_mapper().set_if_empty(&0u32);

        // Parameters
        self.param_manual_claim_mapper().set_if_empty(&false);
        self.param_duration_min_mapper().set_if_empty(&DEFAULT_MIN_DURATION);              
        self.param_duration_max_mapper().set_if_empty(&DEFAULT_MAX_DURATION); 

        // Fees
        self.fee_pool_mapper().set_if_empty(&BigUint::zero());
        self.fee_policy_mapper().set_if_empty(&FeePolicy {
            fee_amount_egld : BigUint::zero(),
            sponsor_reward_percent : 0u8,
        });

        Ok(())
    }

    /////////////////////////////////////////////////////////////////////
    // Administrator API
    /////////////////////////////////////////////////////////////////////

    #[endpoint(triggerEnded)]
    fn trigger_ended_instances(&self) -> SCResult<()> {
        only_owner!(self, "Caller address not allowed");

        let ended_instances: VarArgs<u32> = self.get_instance_ids(MultiArgVec(Vec::from([InstanceStatus::Ended])));

        for iid in ended_instances.iter() {
            self.func_trigger(iid.clone());
        }

        Ok(())
    }

    #[endpoint(cleanClaimed)]
    fn clean_claimed_instances(&self) -> SCResult<()> {
        only_owner!(self, "Caller address not allowed");
        
        let claimed_instances: VarArgs<u32> = self.get_instance_ids(MultiArgVec(Vec::from([InstanceStatus::Claimed])));

        for iid in claimed_instances.iter() {
            self.instance_players_set_mapper(iid.clone()).clear();
            self.instance_players_vec_mapper(iid.clone()).clear();
            self.instance_info_mapper().remove(&iid);
            self.instance_state_mapper().remove(&iid);
        }

        Ok(())
    }  

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

    /////////////////////////////////////////////////////////////////////
    // DApp endpoints : sponsor API
    /////////////////////////////////////////////////////////////////////
    #[payable("*")]
    #[endpoint(create)]
    fn create_instance(&self, #[payment_token] token_identifier: TokenIdentifier, #[payment_nonce] token_nonce: u64, #[payment_amount] token_amount: BigUint, duration_in_s: u64, pseudo: ManagedBuffer, url: ManagedBuffer, logo_link: ManagedBuffer, free_text: ManagedBuffer) -> MultiResult2<SCResult<()>, OptionalResult<u32>> {

        // Check validity of parameters
        require_with_opt!(self.address_blacklist_set_mapper().contains(&self.blockchain().get_caller()) == false, "Caller blacklisted");
        require_with_opt!(duration_in_s >= self.param_duration_min_mapper().get(), "Duration out of allowed range");
        require_with_opt!(duration_in_s <= self.param_duration_max_mapper().get(), "Duration out of allowed range");
        require_with_opt!(token_amount > 0, "Prize cannot be null");

        // Compute next iid
        let new_iid = self.iid_counter_mapper().get() + 1;

        // Aggregate instance information
        let instance_info = InstanceInfo {
            sponsor_info: SponsorInfo {
                address: self.blockchain().get_caller(),
                pseudo: pseudo,
                url: url,
                logo_link: logo_link,
                free_text: free_text,
                reward_percent: self.fee_policy_mapper().get().sponsor_reward_percent},
            prize_info: PrizeInfo {
                prize_type: if token_identifier.is_egld() {PrizeType::EgldPrize} else if token_identifier.is_esdt() {PrizeType::EsdtPrize} else {PrizeType::UnknownPrize},
                token_identifier: token_identifier,
                token_nonce: token_nonce,
                token_amount: token_amount},
            deadline: self.blockchain().get_block_timestamp() + duration_in_s
        };

        // Initialize instance state
        let instance_state = InstanceState {
            sponsor_rewards_pool: BigUint::zero(),
            claimed_status: false,
            winner_info: WinnerInfo {
                ticket_number: 0usize,
                address: ManagedAddress::zero()},
            disabled: false,
        };

        // Record new instance
        self.iid_counter_mapper().set(&new_iid);
        self.instance_info_mapper().insert(new_iid, instance_info);
        self.instance_state_mapper().insert(new_iid, instance_state);

        // Format result
        Ok_some!(new_iid);
    }

    /////////////////////////////////////////////////////////////////////
    // DApp endpoints : player API
    /////////////////////////////////////////////////////////////////////
    #[payable("EGLD")]
    #[endpoint(play)]
    // Returns : Result, optional (ticket number)  
    fn play(&self, #[payment] fees: BigUint, iid: u32) -> MultiResult2<SCResult<()>, OptionalResult<usize>> {

        // Checks
        let caller = self.blockchain().get_caller();
        require_with_opt!(self.address_blacklist_set_mapper().contains(&caller) == false, "Caller blacklisted");
        require_with_opt!(self.get_instance_status(iid) == InstanceStatus::Running, "Instance is not active");
        require_with_opt!(self.instance_players_set_mapper(iid).contains(&caller) == false, "Player has already played");
        require_with_opt!(fees == self.fee_policy_mapper().get().fee_amount_egld, "Wrong fees amount");

        // Capitalize fees and set sponsor rewards
        if fees != BigUint::zero() {
            let instance_info = self.instance_info_mapper().get(&iid).unwrap();
            let mut instance_state = self.instance_state_mapper().get(&iid).unwrap();

            // Compute sponsor rewards
            let sponsor_reward_percent:BigUint = BigUint::from(instance_info.sponsor_info.reward_percent);
            let sponsor_reward_amount: BigUint = fees.clone() * sponsor_reward_percent / BigUint::from(100u8);
            let remaining_fees: BigUint = fees - sponsor_reward_amount.clone();

            // Add rewards to sponsor rewards pool
            instance_state.sponsor_rewards_pool += sponsor_reward_amount;
            self.instance_state_mapper().insert(iid, instance_state);

            // Add fees to pool
            self.fee_pool_mapper().update(|total_fees| *total_fees += remaining_fees);
        }

        // Add caller address to participants for this instance
        self.instance_players_set_mapper(iid).insert(caller.clone());
        self.instance_players_vec_mapper(iid).push(&caller);

        Ok_some!(self.instance_players_vec_mapper(iid).len());
    }

    #[endpoint(claimPrize)]
    fn claim_prize(&self, iid: u32) -> SCResult<()> {
        // Checks
        require!(self.address_blacklist_set_mapper().contains(&self.blockchain().get_caller()) == false, "Caller blacklisted");
        require!(self.get_instance_status(iid) == InstanceStatus::Triggered, "Instance is not in the good state");
        require!(self.blockchain().get_caller() == self.instance_state_mapper().get(&iid).unwrap().winner_info.address, "Prize can only be claimed by the winner");

        // Get prize info & instance state
        let prize_info = self.instance_info_mapper().get(&iid).unwrap().prize_info;
        let mut instance_state = self.instance_state_mapper().get(&iid).unwrap();

        // Send prize to winner address
        self.func_send_prize(&prize_info, &instance_state.winner_info.address);

        // Update claimed status
        instance_state.claimed_status = true;
        self.instance_state_mapper().insert(iid, instance_state);

        Ok(())
    }

    /////////////////////////////////////////////////////////////////////
    // DApp view API
    /////////////////////////////////////////////////////////////////////

    #[view(getNb)]
    fn get_nb_instances(&self) -> u32 {
        return self.instance_info_mapper().len() as u32;
    }

    #[view(getStatus)]
    fn get_instance_status(&self, iid: u32) -> InstanceStatus {
        
        // Compute instance status
        match self.instance_state_mapper().get(&iid) {
            None => return InstanceStatus::NotExisting,
            Some(instance_state) => {
                // Compute instance status based on fields values
                if instance_state.disabled == true {
                    return InstanceStatus::Disabled;
                } else {
                    if instance_state.claimed_status == true {
                        return InstanceStatus::Claimed;
                    } else {
                        if instance_state.winner_info.address != ManagedAddress::zero() {
                            return InstanceStatus::Triggered;
                        } else {
                            let instance_info = self.instance_info_mapper().get(&iid).unwrap();

                            if self.blockchain().get_block_timestamp() > instance_info.deadline {
                                return InstanceStatus::Ended;
                            } else {
                                return InstanceStatus::Running;
                            }
                        }
                    }
                }
            }
        }
    }

    #[view(getInfoLegacy)]
    // Returns : (Result, optional (status, number of players, winner address, instance info)) of instance identified by iid provided  
    fn get_instance_info_legacy(&self, iid: u32) -> MultiResult2<SCResult<()>, OptionalResult<MultiResult4<InstanceStatus, usize, ManagedAddress, InstanceInfo<Self::Api>>>> {
        //Checks
        require_with_opt!(self.get_instance_status(iid) != InstanceStatus::NotExisting, "Instance does not exist");

        Ok_some!(MultiArg4((
            self.get_instance_status(iid),
            self.instance_players_set_mapper(iid).len(),
            self.instance_state_mapper().get(&iid).unwrap().winner_info.address,
            self.instance_info_mapper().get(&iid).unwrap())))
    }   
            
    #[view(getAllInfoLegacy)]
    // Returns : total number of filtered instances followed by, (ID, status, number of players, winner address, instance info) of all filtered instances
    fn get_all_instance_info_legacy(&self, #[var_args] status_filter: VarArgs<InstanceStatus>) -> MultiArg2<usize, VarArgs<MultiArg5<u32, InstanceStatus, usize, ManagedAddress, InstanceInfo<Self::Api>>>> {

        let mut instances: VarArgs<MultiArg5<u32, InstanceStatus, usize, ManagedAddress, InstanceInfo<Self::Api>>> = VarArgs::new();
        let mut status_filter_vec = status_filter.clone().into_vec();

        // Ensure at least one status is provided as filter, check also overflow regarding the maximum possible values for status
        if status_filter.len() >= 1 && status_filter.len() <= InstanceStatus::VARIANT_COUNT {

            // Remove duplicates
            status_filter_vec.sort();
            status_filter_vec.dedup();

            // Return all instances IDs which meet the status filter provided in parameter
            for iid in self.instance_info_mapper().keys() {
                for status in status_filter_vec.iter() {
                    if self.get_instance_status(iid) == status.clone() {
                        let result_vec_item = MultiArg5((
                            iid,
                            self.get_instance_status(iid), 
                            self.instance_players_set_mapper(iid).len(),
                            self.instance_state_mapper().get(&iid).unwrap().winner_info.address,
                            self.instance_info_mapper().get(&iid).unwrap(),
                        ));
                        instances.push(result_vec_item);
                        break;
                    }   
                }
            }
        }

        return MultiArg2((instances.len(), instances));
    }

    #[view(getInfo)]
    fn get_instance_info(&self, iid: u32) -> MultiResult2<SCResult<()>, OptionalResult<GetInfoStruct<Self::Api>>> {
        //Checks
        require_with_opt!(self.get_instance_status(iid) != InstanceStatus::NotExisting, "Instance does not exist");

        let instance_info = self.instance_info_mapper().get(&iid).unwrap();

        Ok_some!(GetInfoStruct {
            iid: iid,
            instance_status: self.get_instance_status(iid),
            number_of_players: self.instance_players_set_mapper(iid).len(),
            winner_info: self.instance_state_mapper().get(&iid).unwrap().winner_info,
            sponsor_info: instance_info.sponsor_info,
            prize_info: instance_info.prize_info,
            deadline: instance_info.deadline})
    }   
            
    #[view(getAllInfo)]
    // Returns : total number of filtered instances followed by information of all filtered instances
    fn get_all_instance_info(&self, #[var_args] status_filter: VarArgs<InstanceStatus>) -> MultiArg2<usize, VarArgs<GetInfoStruct<Self::Api>>> {

        let mut instances: VarArgs<GetInfoStruct<Self::Api>> = VarArgs::new();
        let mut status_filter_vec = status_filter.clone().into_vec();

        // Ensure at least one status is provided as filter, check also overflow regarding the maximum possible values for status
        if status_filter.len() >= 1 && status_filter.len() <= InstanceStatus::VARIANT_COUNT {

            // Remove duplicates
            status_filter_vec.sort();
            status_filter_vec.dedup();

            // Return all instances IDs which meet the status filter provided in parameter
            for iid in self.instance_info_mapper().keys() {
                for status in status_filter_vec.iter() {
                    if self.get_instance_status(iid) == status.clone() {
                        instances.push(self.get_instance_info(iid).0.1.into_option().unwrap());
                        break;
                    }   
                }
            }
        }

        return MultiArg2((instances.len(), instances));
    }

    #[view(getRemainingTime)]
    fn get_remaining_time(&self, iid: u32) -> MultiResult2<SCResult<()>, OptionalResult<u64>> {
        require_with_opt!(self.get_instance_status(iid) != InstanceStatus::NotExisting, "Instance does not exist");

        // Compute remaining time
        let deadline: u64 = self.instance_info_mapper().get(&iid).unwrap().deadline;
        let current_date_time: u64 = self.blockchain().get_block_timestamp();
        let mut remaining_time: u64 = 0;

        if deadline > current_date_time {
            remaining_time = deadline - current_date_time;
        }

        Ok_some!(remaining_time);
    }

    #[view(hasStatus)]
    fn is_instance_with_status(&self, instance_status: InstanceStatus) -> bool {
        let instances: VarArgs<u32> = self.get_instance_ids(MultiArgVec(Vec::from([instance_status])));
        return instances.len() != 0;
    }

    #[view(getIDs)]
    fn get_instance_ids(&self, #[var_args] status_filter: VarArgs<InstanceStatus>) -> VarArgs<u32> {

        let mut instance_ids = VarArgs::new();
        let mut status_filter_vec = status_filter.clone().into_vec();

        // Ensure at least one status is provided as filter, check also overflow regarding the maximum possible values for status
        if status_filter.len() >= 1 && status_filter.len() <= InstanceStatus::VARIANT_COUNT {

            // Remove duplicates
            status_filter_vec.sort();
            status_filter_vec.dedup();

            // Return all instances IDs which meet the status filter provided in parameter
            for iid in self.instance_info_mapper().keys() {
                for status in status_filter_vec.iter() {
                    if self.get_instance_status(iid) == status.clone() {
                        instance_ids.push(iid.clone());
                        break;
                    }   
                }
            }
        }

        return instance_ids;
    }

    #[view(getSponsorIDs)]
    fn get_sponsor_instances(&self, sponsor_address: ManagedAddress) -> VarArgs<u32> {
        let mut sponsor_iids = VarArgs::new();

        // Return all instances IDs with sponsor address matching the one provided in parameter
        for instance in self.instance_info_mapper().iter() {
            if instance.1.sponsor_info.address.clone() == sponsor_address {
                sponsor_iids.push(instance.0);
            }
        }

        return sponsor_iids;
    }

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

    #[view(hasWon)]
    fn has_won(&self, iid: u32, player_address: ManagedAddress) -> MultiResult2<SCResult<()>, OptionalResult<bool>> {
        // Checks
        require_with_opt!(self.get_instance_status(iid) != InstanceStatus::NotExisting, "Instance does not exist");

        let mut result: bool = false;

        if player_address == self.instance_state_mapper().get(&iid).unwrap().winner_info.address {
            result = true;
        }

        Ok_some!(result)
    }

    /////////////////////////////////////////////////////////////////////
    // Private functions
    /////////////////////////////////////////////////////////////////////
    fn func_send_prize(&self, prize_info: &PrizeInfo<Self::Api>, winner_address: &ManagedAddress) {

        // Send prize to winner address
        self.send().direct(
            winner_address,
            &prize_info.token_identifier,
            prize_info.token_nonce,
            &prize_info.token_amount,
            b"Send prize",
        );
    }

    fn func_trigger(&self, iid: u32) -> SCResult<()> {

        // Check validity of parameters
        require!(self.get_instance_status(iid) == InstanceStatus::Ended, "Instance is not in the good state");
        
        // Get instance info & state
        let instance_info = self.instance_info_mapper().get(&iid).unwrap();
        let mut instance_state = self.instance_state_mapper().get(&iid).unwrap();

        // Send rewards to sponsor
        if instance_state.sponsor_rewards_pool > BigUint::zero() {
            self.send().direct_egld(
                &instance_info.sponsor_info.address,
                &instance_state.sponsor_rewards_pool,
                b"Sponsor rewards",
            );
        }

        if self.instance_players_vec_mapper(iid).len() == 0 {
            // No player, give prize back to instance sponsor
            instance_state.winner_info.address = instance_info.sponsor_info.address.clone();
        } 
        else {
            // Choose winner
            let mut rand = RandomnessSource::<Self::Api>::new();       
            let winner_address_index = rand.next_usize_in_range(1, self.instance_players_vec_mapper(iid).len() + 1);
            instance_state.winner_info.ticket_number = winner_address_index.clone();
            instance_state.winner_info.address = self.instance_players_vec_mapper(iid).get(winner_address_index);
        }

        // Prize auto-distribution if enabled
        if self.param_manual_claim_mapper().get() == false {
            // Send prize to winner address
            self.func_send_prize(&instance_info.prize_info, &instance_state.winner_info.address);

            // Update claimed status
            instance_state.claimed_status = true;
        }

        // Record new instance state
        self.instance_state_mapper().insert(iid, instance_state);          

        Ok(())
    }

    /////////////////////////////////////////////////////////////////////
    // Mappers
    /////////////////////////////////////////////////////////////////////

    // Instance counter
    #[storage_mapper("iid_counter")]
    fn iid_counter_mapper(&self) -> SingleValueMapper<u32>;

    // Instance info & state
    #[storage_mapper("instance_info")]
    fn instance_info_mapper(&self) -> MapMapper<u32, InstanceInfo<Self::Api>>;
    #[storage_mapper("instance_state")]
    fn instance_state_mapper(&self) -> MapMapper<u32, InstanceState<Self::Api>>;

    // Instance players
    #[storage_mapper("instance_players_set")]
    fn instance_players_set_mapper(&self, iid: u32) -> SetMapper<ManagedAddress>;
    #[storage_mapper("instance_players_vec")]
    fn instance_players_vec_mapper(&self, iid: u32) -> VecMapper<ManagedAddress>;
}
