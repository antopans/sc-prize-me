#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

////////////////////////////////////////////////////////////////////
// Modules & uses
////////////////////////////////////////////////////////////////////
mod instance;
mod player;
mod security;
mod parameter;
mod fee;
mod event;
mod macros;

use instance::*;

////////////////////////////////////////////////////////////////////
// Types
////////////////////////////////////////////////////////////////////

// data format for endpoint return
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct GetInfoStruct<M: ManagedTypeApi> {
    pub iid: u32,
    pub instance_status: InstanceStatus,
    pub number_of_players: usize,
    pub winner_info: WinnerInfo<M>,
    pub sponsor_info: SponsorInfo<M>,
    pub prize_info: PrizeInfo<M>,
    pub deadline: u64,
}

////////////////////////////////////////////////////////////////////
// Functions
////////////////////////////////////////////////////////////////////
#[elrond_wasm::contract]
pub trait Prize: 
    instance::InstanceModule
    +player::PlayerModule
    +security::SecurityModule 
    +parameter::ParameterModule
    +fee::FeeModule
    +event::EventModule {
    
    /////////////////////////////////////////////////////////////////////
    // SC Management endpoints
    /////////////////////////////////////////////////////////////////////

    #[init]
    fn init(&self) -> SCResult<()> {
        const DEFAULT_MIN_DURATION: u64 = 60;           // 60 seconds
        const DEFAULT_MAX_DURATION: u64 = 60*60*24*365; // 1 year
        const DEFAULT_MAX_NB_INSTANCES_PER_SPONSOR: u32 = 20;
        const DEFAULT_FEE_AMOUNT_EGLD: u32 = 0;
        const DEFAULT_SPONSOR_REWARD_PERCENT: u8 = 0u8;
        
        // Initializations @ deployment only 

        // Instances
        self.iid_counter_mapper().set_if_empty(&0u32);

        // Parameters
        self.param_manual_claim_mapper().set_if_empty(&false);
        self.param_nb_max_instances_per_sponsor_mapper().set_if_empty(&DEFAULT_MAX_NB_INSTANCES_PER_SPONSOR);
        self.param_duration_min_mapper().set_if_empty(&DEFAULT_MIN_DURATION);              
        self.param_duration_max_mapper().set_if_empty(&DEFAULT_MAX_DURATION); 

        // Fees
        self.init_fees_if_empty(BigUint::from(DEFAULT_FEE_AMOUNT_EGLD), DEFAULT_SPONSOR_REWARD_PERCENT);

        // Event
        self.log_enable_mapper().set_if_empty(&false);

        Ok(())
    }

    /////////////////////////////////////////////////////////////////////
    // Administrator endpoints
    /////////////////////////////////////////////////////////////////////

    #[endpoint(triggerEnded)]
    fn trigger_ended_instances(&self) -> SCResult<()> {
        only_owner!(self, "Caller address not allowed");

        let ended_instances: VarArgs<u32> = self.get_instance_ids(MultiArgVec(Vec::from([InstanceStatus::Ended])));

        for iid in ended_instances.iter() {
            // Get instance info & state
            let instance_info = self.instance_info_mapper().get(&iid).unwrap();
            let mut instance_state = self.instance_state_mapper().get(&iid).unwrap();

            // Send rewards to sponsor
            self.pay_rewards_to_sponsor(iid.clone(), instance_info.sponsor_info.address.clone(), instance_state.reward_info.pool.clone());

            if self.instance_players_vec_mapper(iid.clone()).len() == 0 {
                // No player, give prize back to instance sponsor
                instance_state.winner_info.address = instance_info.sponsor_info.address.clone();
            } 
            else {
                // Choose winner
                let mut rand = RandomnessSource::<Self::Api>::new();       
                let winner_address_index = rand.next_usize_in_range(1, self.instance_players_vec_mapper(iid.clone()).len() + 1);
                instance_state.winner_info.ticket_number = winner_address_index.clone();
                instance_state.winner_info.address = self.instance_players_vec_mapper(iid.clone()).get(winner_address_index);
            }

            // Auto-distribution of prize if enabled
            if self.param_manual_claim_mapper().get() == false {
                // Send prize to winner address
                self.func_send_prize(&instance_info.prize_info, &instance_state.winner_info.address);

                // Update claimed status
                instance_state.claimed_status = true;

                // Log event
                self.event_wrapper_auto_claim_prize(iid.clone());
            }
            
            // Log event
            self.event_wrapper_trigger(iid.clone(), instance_state.winner_info.ticket_number, &instance_state.winner_info.address);
            
            // Record new instance state
            self.instance_state_mapper().insert(iid.clone(), instance_state);   
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

            // Log event
            self.event_wrapper_clean_claim(iid.clone());
        }

        Ok(())
    }  

    /////////////////////////////////////////////////////////////////////
    // DApp endpoints : sponsor API
    /////////////////////////////////////////////////////////////////////
    #[payable("*")]
    #[endpoint(create)]
    fn create_instance(&self, #[payment_token] token_identifier: TokenIdentifier, #[payment_nonce] token_nonce: u64, #[payment_amount] token_amount: BigUint, duration_in_s: u64, pseudo: ManagedBuffer, url: ManagedBuffer, logo_link: ManagedBuffer, free_text: ManagedBuffer) -> MultiResult2<SCResult<()>, OptionalResult<u32>> {

        // Check validity of parameters
        let caller = self.blockchain().get_caller();
        require_with_opt!(self.address_blacklist_set_mapper().contains(&caller) == false, "Caller blacklisted");
        require_with_opt!(self.get_nb_sponsor_running(caller.clone()) < self.get_param_nb_max_instances_per_sponsor(), "Max instances reached for this sponsor");
        require_with_opt!(duration_in_s >= self.param_duration_min_mapper().get(), "Duration out of allowed range");
        require_with_opt!(duration_in_s <= self.param_duration_max_mapper().get(), "Duration out of allowed range");
        require_with_opt!(token_amount > 0, "Prize cannot be null");

        // Compute next iid
        let new_iid = self.iid_counter_mapper().get() + 1;

        // Aggregate instance information
        let instance_info = InstanceInfo {
            sponsor_info: SponsorInfo {
                address: caller.clone(),
                pseudo: pseudo.clone(),
                url: url.clone(),
                logo_link: logo_link.clone(),
                free_text: free_text.clone()},
            prize_info: PrizeInfo {
                prize_type: if token_identifier.is_egld() {PrizeType::EgldPrize} else if token_identifier.is_esdt() {PrizeType::EsdtPrize} else {PrizeType::UnknownPrize},
                token_identifier: token_identifier.clone(),
                token_nonce: token_nonce,
                token_amount: token_amount.clone()},
            deadline: self.blockchain().get_block_timestamp() + duration_in_s
        };

        // Initialize instance state
        let instance_state = InstanceState {
            claimed_status: false,
            reward_info: RewardInfo {
                percent: self.fee_policy_mapper().get().sponsor_reward_percent,
                pool: BigUint::zero()},
            winner_info: WinnerInfo {
                ticket_number: 0usize,
                address: ManagedAddress::zero()},
            disabled: false,
        };

        // Record new instance
        self.iid_counter_mapper().set(&new_iid);
        self.instance_info_mapper().insert(new_iid, instance_info);
        self.instance_state_mapper().insert(new_iid, instance_state);

        // Log event
        self.event_wrapper_create_instance(&caller, new_iid, &token_identifier, token_nonce, &token_amount, duration_in_s, &pseudo);

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

        // Capitalize fees and sponsor rewards
        let mut instance_state = self.instance_state_mapper().get(&iid).unwrap();
        instance_state.reward_info.pool += self.update_fees_and_compute_rewards(fees.clone(), BigUint::from(instance_state.reward_info.percent));
        self.event_wrapper_reward_pool_info(iid, &instance_state.reward_info.pool); 
        self.instance_state_mapper().insert(iid, instance_state);
        
        // Add caller address to participants for this instance
        self.instance_players_set_mapper(iid).insert(caller.clone());
        self.instance_players_vec_mapper(iid).push(&caller);
        let ticket_number: usize = self.instance_players_vec_mapper(iid).len();

        // Log event
        self.event_wrapper_play(&caller, iid, ticket_number, &fees);

        Ok_some!(ticket_number);
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

        // Log event
        self.event_wrapper_manual_claim_prize(iid);

        Ok(())
    }

    /////////////////////////////////////////////////////////////////////
    // DApp view API
    /////////////////////////////////////////////////////////////////////

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

    /////////////////////////////////////////////////////////////////////
    // Internal SC functions
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

    fn pay_rewards_to_sponsor(&self, iid: u32, sponsor_address: ManagedAddress, rewards: BigUint) {

        if rewards > BigUint::zero() {
            self.send().direct_egld(
                &sponsor_address,
                &rewards,
                b"Sponsor rewards",
            );
            
            // Log event
            self.event_wrapper_send_rewards(iid, &rewards);
        }
    }

}
