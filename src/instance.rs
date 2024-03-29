elrond_wasm::imports!();
elrond_wasm::derive_imports!();

////////////////////////////////////////////////////////////////////
// Modules & uses
////////////////////////////////////////////////////////////////////
extern crate variant_count;
use variant_count::VariantCount;

use super::Ok_some;
use super::require_with_opt;
use super::event;

////////////////////////////////////////////////////////////////////
// Types
////////////////////////////////////////////////////////////////////
#[derive(ManagedVecItem, NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, PartialEq, Clone, Copy, VariantCount, Ord, PartialOrd, Eq)]
pub enum InstanceStatus {
    NotExisting,
    Running,
    Ended,
    Triggered,
    Claimed,
    Disabled,
}

// Information filled at instance creation
#[derive(ManagedVecItem, NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct SponsorInfo<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub pseudo: ManagedBuffer<M>,
    pub url1: ManagedBuffer<M>,
    pub url2: ManagedBuffer<M>,
    pub url3: ManagedBuffer<M>,
    pub reserved: ManagedBuffer<M>,
    pub graphic: ManagedBuffer<M>,
    pub logo_link: ManagedBuffer<M>,
    pub free_text: ManagedBuffer<M>,
}

#[derive(ManagedVecItem, NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct PrizeInfo<M: ManagedTypeApi> {
    pub token_identifier: TokenIdentifier<M>,
    pub token_nonce: u64,
    pub token_amount: BigUint<M>,
}

#[derive(ManagedVecItem, NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct InstanceInfo<M: ManagedTypeApi> {
    pub sponsor_info: SponsorInfo<M>,
    pub prize_info: PrizeInfo<M>,
    pub premium: bool,
    pub charity: bool,
    pub deadline: u64,
}

// State of instance, content depends on instance lifecycle
#[derive(ManagedVecItem, NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct RewardInfo<M: ManagedTypeApi> {
    pub percent: u8,
    pub pool: BigUint<M>,
}

#[derive(ManagedVecItem, NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct WinnerInfo<M: ManagedTypeApi> {
    pub ticket_number: usize,
    pub address: ManagedAddress<M>,
}

#[derive(ManagedVecItem, NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct InstanceState<M: ManagedTypeApi> {
    pub claimed_status: bool,
    pub reward_info: RewardInfo<M>,
    pub winner_info: WinnerInfo<M>,
    pub disabled: bool,
}

////////////////////////////////////////////////////////////////////
// Functions
////////////////////////////////////////////////////////////////////
#[elrond_wasm::module]
pub trait InstanceModule:
    event::EventModule {

    /////////////////////////////////////////////////////////////////////
    // Endpoints
    /////////////////////////////////////////////////////////////////////
    #[only_owner]
    #[endpoint(setPremium)]
    fn set_premium(&self, iid: u32, premium_status: bool) -> SCResult<()> {    
        //Checks
        require!(self.get_instance_status(iid) != InstanceStatus::NotExisting, "Instance does not exist");

        // Update premium status
        let mut instance_info = self.instance_info_mapper().get(&iid).unwrap();
        instance_info.premium = premium_status;
        self.instance_info_mapper().insert(iid, instance_info);

        // Log event
        self.event_wrapper_set_premium(iid, premium_status);

        Ok(())
    }

    /////////////////////////////////////////////////////////////////////
    // Queries
    /////////////////////////////////////////////////////////////////////
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

    #[view(hasStatus)]
    fn is_instance_with_status(&self, instance_status: InstanceStatus) -> bool {
        let instances: MultiValueManagedVec<u32> = self.get_instance_ids(MultiValueManagedVec::from_single_item(instance_status));
        return instances.len() != 0;
    }

    #[view(getNb)]
    fn get_nb_instances(&self, #[var_args] status_filter: MultiValueManagedVec<InstanceStatus>) -> u32 {
        let mut nb_instances: u32 = 0;

        // Check overflow regarding the maximum possible values for status
        if status_filter.len() <= InstanceStatus::VARIANT_COUNT {

            if status_filter.len() == 0 {
                // No filter provided, return the total number of instances, whatever the status of each instance
                nb_instances = self.instance_info_mapper().len() as u32;
            }
            else {
                // Return the total number of instances which meet the status filter provided in parameter
                for iid in self.instance_info_mapper().keys() {
                    for status in status_filter.iter() {
                        if self.get_instance_status(iid) == status.clone() {
                            nb_instances += 1;
                            break;
                        }   
                    }
                }
            }
        }

        return nb_instances;
    }

    #[view(getRemainingTime)]
    fn get_remaining_time(&self, iid: u32) -> MultiValue2<SCResult<()>, OptionalValue<u64>> {
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

    #[view(getIDs)]
    fn get_instance_ids(&self, #[var_args] status_filter: MultiValueManagedVec<InstanceStatus>) -> MultiValueManagedVec<u32> {

        let mut instance_ids = MultiValueManagedVec::new();
        // let mut status_filter_vec = status_filter.clone().into_vec().into_vec();

        // Ensure at least one status is provided as filter, check also overflow regarding the maximum possible values for status
        if status_filter.len() >= 1 && status_filter.len() <= InstanceStatus::VARIANT_COUNT {

            // Remove duplicates
            // status_filter_vec.sort();
            // status_filter_vec.dedup();

            // Return all instances IDs which meet the status filter provided in parameter
            for iid in self.instance_info_mapper().keys() {
                for status in status_filter.iter() {
                    if self.get_instance_status(iid) == status.clone() {
                        instance_ids.push(iid.clone());
                        break;
                    }   
                }
            }
        }

        return instance_ids;
    }

    #[view(hasWon)]
    fn has_won(&self, iid: u32, player_address: ManagedAddress) -> MultiValue2<SCResult<()>, OptionalValue<bool>> {
        // Checks
        require_with_opt!(self.get_instance_status(iid) != InstanceStatus::NotExisting, "Instance does not exist");

        let mut result: bool = false;

        if player_address == self.instance_state_mapper().get(&iid).unwrap().winner_info.address {
            result = true;
        }

        Ok_some!(result)
    }

    /////////////////////////////////////////////////////////////////////
    // Mappers
    /////////////////////////////////////////////////////////////////////
    
    // Instance counter
    #[storage_mapper("iid_counter")]
    fn iid_counter_mapper(&self) -> SingleValueMapper<u32>;

    // Instance info
    #[storage_mapper("instance_info")]
    fn instance_info_mapper(&self) -> MapMapper<u32, InstanceInfo<Self::Api>>;

    // Instance state
    #[storage_mapper("instance_state")]
    fn instance_state_mapper(&self) -> MapMapper<u32, InstanceState<Self::Api>>;
}