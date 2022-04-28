elrond_wasm::imports!();

/////////////////////////////////////////////////////////////////////
// Functions
/////////////////////////////////////////////////////////////////////
#[elrond_wasm::module]
pub trait EventModule {

    /////////////////////////////////////////////////////////////////////
    // Endpoints
    /////////////////////////////////////////////////////////////////////
    #[only_owner]
    #[endpoint(setLogEnableStatus)]
    fn set_log_enable_status(&self, enable: bool) -> SCResult<()> {       
        self.log_enable_mapper().update(|current_value| *current_value = enable);
        Ok(())
    }

    /////////////////////////////////////////////////////////////////////
    // Queries
    /////////////////////////////////////////////////////////////////////
    #[view(getLogEnableStatus)]
    fn get_log_enable_status(&self) -> bool {        
        return self.log_enable_mapper().get(); 
    }

    /////////////////////////////////////////////////////////////////////
    // Event wrappers
    /////////////////////////////////////////////////////////////////////
     
    // Events occuring during instance lifecycle
    fn event_wrapper_create_instance(
        &self,
        sponsor_address: &ManagedAddress,
        iid: u32,
        token_identifier: &TokenIdentifier,
        token_nonce: u64,
        token_amount: &BigUint,
        duration_in_s: u64,
        pseudo: &ManagedBuffer
    ) {
        if self.log_enable_mapper().get() == true {
            self.create_instance_event(
                self.blockchain().get_block_epoch(),
                sponsor_address,
                iid,
                token_identifier,
                token_nonce,
                token_amount,
                duration_in_s,
                pseudo
            );
        }
    }

    fn event_wrapper_play(
        &self,
        player_address: &ManagedAddress,
        iid: u32,
        ticket_number: usize,
        fees: &BigUint
    ) {
        if self.log_enable_mapper().get() == true {
            self.play_event(
                self.blockchain().get_block_epoch(),
                player_address,
                iid,
                ticket_number,
                fees
            );
        }
    }

    fn event_wrapper_trigger(
        &self,
        iid: u32,
        ticket_number: usize,
        winner_address: &ManagedAddress
    ) {
        if self.log_enable_mapper().get() == true {
            self.trigger_event(
                self.blockchain().get_block_epoch(),
                iid,
                ticket_number,
                winner_address
            );
        }
    }

    fn event_wrapper_fee_pool_info(
        &self,
        fee_pool: &BigUint
    ) {
        if self.log_enable_mapper().get() == true {
            self.fee_pool_info_event(
                self.blockchain().get_block_epoch(),
                fee_pool
            );
        }
    }

    fn event_wrapper_reward_pool_info(
        &self,
        iid: u32,
        reward_pool: &BigUint
    ) {
        if self.log_enable_mapper().get() == true {
            self.reward_pool_info_event(
                self.blockchain().get_block_epoch(),
                iid,
                reward_pool
            );
        }
    }

    fn event_wrapper_send_rewards(
        &self,
        iid: u32,
        rewards: &BigUint
    ) {
        if self.log_enable_mapper().get() == true {
            self.send_rewards_event(
                self.blockchain().get_block_epoch(),
                iid,
                rewards
            );
        }
    }

    fn event_wrapper_auto_claim_prize(
        &self,
        iid: u32
    ) {
        if self.log_enable_mapper().get() == true {
            self.auto_claim_prize_event(
                self.blockchain().get_block_epoch(),
                iid
            );
        }
    }

    fn event_wrapper_claim_fees(
        &self,
        fee_amount: &BigUint
    ) {
        if self.log_enable_mapper().get() == true {
            self.claim_fees_event(
                self.blockchain().get_block_epoch(),
                fee_amount
            );
        }
    }

    fn event_wrapper_claim_link_rewards(
        &self,
        reward_amount: &BigUint,
        link_address: &ManagedAddress
    ) {
        if self.log_enable_mapper().get() == true {
            self.claim_link_rewards_event(
                self.blockchain().get_block_epoch(),
                reward_amount,
                link_address
            );
        }
    }

    fn event_wrapper_claim_donations(
        &self,
        donations_amount: &BigUint
    ) {
        if self.log_enable_mapper().get() == true {
            self.claim_donations_event(
                self.blockchain().get_block_epoch(),
                donations_amount
            );
        }
    }

    fn event_wrapper_manual_claim_prize(
        &self,
        iid: u32
    ) {
        if self.log_enable_mapper().get() == true {
            self.manual_claim_prize_event(
                self.blockchain().get_block_epoch(),
                iid
            );
        }
    }

    fn event_wrapper_clean_claim(
        &self,
        iid: u32
    ) {
        if self.log_enable_mapper().get() == true {
            self.clean_claim_event(
                self.blockchain().get_block_epoch(),
                iid
            );
        }
    }

    // Events occuring during setup
    fn event_wrapper_set_premium(
        &self,
        iid: u32,
        premium_status: bool
    ) {
        if self.log_enable_mapper().get() == true {
            self.set_premium_event(
                self.blockchain().get_block_epoch(),
                iid,
                premium_status
            );
        }
    }

    fn event_wrapper_disable_instance(
        &self,
        iid: u32,
        disable_status: bool
    ) {
        if self.log_enable_mapper().get() == true {
            self.disable_instance_event(
                self.blockchain().get_block_epoch(),
                iid,
                disable_status
            );
        }
    }

    fn event_wrapper_set_fee_policy(
        &self,
        fee_amount_egld: &BigUint,
        sponsor_reward_percent: u8,
        link_reward_percent: u8
    ) {
        if self.log_enable_mapper().get() == true {
            self.set_fee_policy_event(
                self.blockchain().get_block_epoch(),
                fee_amount_egld,
                sponsor_reward_percent,
                link_reward_percent
            );
        }
    }

    fn event_wrapper_set_param_duration(
        &self,
        duration_min: u64,
        duration_max: u64
    ) {
        if self.log_enable_mapper().get() == true {
            self.set_param_duration_event(
                self.blockchain().get_block_epoch(),
                duration_min,
                duration_max
            );
        }
    }

    fn event_wrapper_set_param_sponsor_info_max_length(
        &self,
        length_max: u32
    ) {
        if self.log_enable_mapper().get() == true {
            self.set_param_sponsor_info_max_length_event(
                self.blockchain().get_block_epoch(),
                length_max
            );
        }
    }

    fn event_wrapper_set_param_nb_max_instances_per_sponsor(
        &self,
        nb_instances_max: u32
    ) {
        if self.log_enable_mapper().get() == true {
            self.set_param_nb_max_instances_per_sponsor_event(
                self.blockchain().get_block_epoch(),
                nb_instances_max
            );
        }
    }

    fn event_wrapper_set_param_manual_claim(
        &self,
        manual_claim: bool
    ) {
        if self.log_enable_mapper().get() == true {
            self.set_param_manual_claim_event(
                self.blockchain().get_block_epoch(),
                manual_claim
            );
        }
    }

    fn event_wrapper_add_addr_blacklist(
        &self,
        address: &ManagedAddress
    ) {
        if self.log_enable_mapper().get() == true {
            self.add_addr_blacklist_event(
                self.blockchain().get_block_epoch(),
                address
            );
        }
    }

    fn event_wrapper_rm_addr_blacklist(
        &self,
        address: &ManagedAddress
    ) {
        if self.log_enable_mapper().get() == true {
            self.rm_addr_blacklist_event(
                self.blockchain().get_block_epoch(),
                address
            );
        }
    }

    /////////////////////////////////////////////////////////////////////
    // Events
    /////////////////////////////////////////////////////////////////////
     
    // Events occuring during instance lifecycle
    #[event("create_instance")]
    fn create_instance_event(
        &self,
        #[indexed] epoch: u64,
        #[indexed] sponsor_address: &ManagedAddress,
        #[indexed] iid: u32,
        #[indexed] token_identifier: &TokenIdentifier,
        #[indexed] token_nonce: u64,
        #[indexed] token_amount: &BigUint,
        #[indexed] duration_in_s: u64,
        #[indexed] pseudo: &ManagedBuffer
    ); 

    #[event("play")]
    fn play_event(
        &self,
        #[indexed] epoch: u64,
        #[indexed] player_address: &ManagedAddress,
        #[indexed] iid: u32,
        #[indexed] ticket_number: usize,
        #[indexed] fees: &BigUint
    ); 

    #[event("trigger")]
    fn trigger_event(
        &self,
        #[indexed] epoch: u64,
        #[indexed] iid: u32,
        #[indexed] ticket_number: usize,
        #[indexed] winner_address: &ManagedAddress
    ); 

    #[event("fee_pool_info")]
    fn fee_pool_info_event(
        &self,
        #[indexed] epoch: u64,
        #[indexed] fees: &BigUint
    );

    #[event("reward_pool_info")]
    fn reward_pool_info_event(
        &self,
        #[indexed] epoch: u64,
        #[indexed] iid: u32,
        #[indexed] reward_pool: &BigUint
    );

    #[event("send_rewards")]
    fn send_rewards_event(
        &self,
        #[indexed] epoch: u64,
        #[indexed] iid: u32,
        #[indexed] rewards: &BigUint,
    ); 

    #[event("auto_claim_prize")]
    fn auto_claim_prize_event(
        &self,
        #[indexed] epoch: u64,
        #[indexed] iid: u32
    ); 

    #[event("claim_fees")]
    fn claim_fees_event(
        &self,
        #[indexed] epoch: u64,
        #[indexed] fee_amount: &BigUint
    ); 

    #[event("claim_link_rewards")]
    fn claim_link_rewards_event(
        &self,
        #[indexed] epoch: u64,
        #[indexed] reward_amount: &BigUint,
        #[indexed] link_address: &ManagedAddress
    ); 

    #[event("claim_donations")]
    fn claim_donations_event(
        &self,
        #[indexed] epoch: u64,
        #[indexed] donations_amount: &BigUint
    ); 

    #[event("manual_claim_prize")]
    fn manual_claim_prize_event(
        &self,
        #[indexed] epoch: u64,
        #[indexed] iid: u32
    ); 

    #[event("clean_claim")]
    fn clean_claim_event(
        &self,
        #[indexed] epoch: u64,
        #[indexed] iid: u32
    ); 

    // Events occuring during setup
    #[event("set_premium")]
    fn set_premium_event(
        &self,
        #[indexed] epoch: u64,
        #[indexed] iid: u32,
        #[indexed] premium_status: bool
    ); 

    #[event("disable_instance")]
    fn disable_instance_event(
        &self,
        #[indexed] epoch: u64,
        #[indexed] iid: u32,
        #[indexed] disable_status: bool
    ); 

    #[event("set_fee_policy")]
    fn set_fee_policy_event(
        &self,
        #[indexed] epoch: u64,
        #[indexed] fee_amount_egld: &BigUint,
        #[indexed] sponsor_reward_percent: u8,
        #[indexed] link_reward_percent: u8
    ); 

    #[event("set_param_duration")]
    fn set_param_duration_event(
        &self,
        #[indexed] epoch: u64,
        #[indexed] duration_min: u64,
        #[indexed] duration_max: u64
    ); 

    #[event("set_param_sponsor_info_max_length")]
    fn set_param_sponsor_info_max_length_event(
        &self,
        #[indexed] epoch: u64,
        #[indexed] length_max: u32
    ); 

    #[event("set_param_nb_max_instances_per_sponsor")]
    fn set_param_nb_max_instances_per_sponsor_event(
        &self,
        #[indexed] epoch: u64,
        #[indexed] nb_instances_max: u32
    ); 

    #[event("set_param_manual_claim")]
    fn set_param_manual_claim_event(
        &self,
        #[indexed] epoch: u64,
        #[indexed] manual_claim: bool
    ); 

    #[event("add_addr_blacklist")]
    fn add_addr_blacklist_event(
        &self,
        #[indexed] epoch: u64,
        #[indexed] address: &ManagedAddress
    ); 

    #[event("rm_addr_blacklist")]
    fn rm_addr_blacklist_event(
        &self,
        #[indexed] epoch: u64,
        #[indexed] address: &ManagedAddress
    ); 

    /////////////////////////////////////////////////////////////////////
    // Mappers
    /////////////////////////////////////////////////////////////////////
    
    // Log enable status
    #[storage_mapper("log_enable")]
    fn log_enable_mapper(&self) -> SingleValueMapper<bool>;
}

