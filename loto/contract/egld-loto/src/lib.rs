#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(clippy::too_many_arguments)]

elrond_wasm::imports!();

use elrond_wasm::String;

mod instance_info;
mod instance_status;
mod random;

use instance_info::OwnerInfo;
use instance_info::InstanceInfo;
use instance_status::InstanceStatus;
use random::Random;

#[elrond_wasm_derive::contract]

pub trait Loto {
	/////////////////////////////////////////////////////////////////////
	// Administrator API
	/////////////////////////////////////////////////////////////////////
	
	#[init]
	fn init(&self, fee_percent: Self::BigUint) -> SCResult<()> {
		// Setup fee
		require!(fee_percent<=100, "fee value can't be higher than 100");
		self.set_fee(fee_percent);

		// Initialize instance ID
		self.set_iid_cpt(0);		

		Ok(())
	}

	#[endpoint(updateFee)]
	fn update_fee(&self, fee_percent: Self::BigUint) -> SCResult<()> {
		only_owner!(self, "Caller address not allowed");

		// Setup fee
		require!(fee_percent<=100, "fee value can't be higher than 100");
		self.set_fee(fee_percent);

		Ok(())
	}

	#[view(getFee)]
	#[storage_get("Fee")]
	fn get_fee(&self) -> Self::BigUint;

	#[endpoint(triggerEndedInstances)]
	fn trigger_ended_instances(&self) -> SCResult<()> {
		only_owner!(self, "Caller address not allowed");
		// TODO
		Ok(())
	}

	#[view(checkEndedInstances)]
	fn check_ended_instances(&self) -> bool {
		// TODO
		true
	}

	#[endpoint(cleanClaimedInstances)]
	fn clean_claimed_instances(&self) -> SCResult<()> {
		only_owner!(self, "Caller address not allowed");
		// TODO
		Ok(())
	}

	/////////////////////////////////////////////////////////////////////
	// DApp endpoints : customer API
	/////////////////////////////////////////////////////////////////////
	#[payable("EGLD")]
	#[endpoint(createInstance)]
	//fn create_instance(&self, #[payment] egld_amount: Self::BigUint, duration_in_s: u64, pseudo: String, url: String, picture_link: String, free_text: String) -> MultiResult2<SCResult<()>, u32>  {
	fn create_instance(&self, #[payment] egld_amount: Self::BigUint, duration_in_s: u64) -> MultiResult2<SCResult<()>, u32>  {
		let mut result;
		let mut prize: Self::BigUint;
		let caller = self.blockchain().get_caller();
		
		// Check validity of parameters 
		// TODO : add min egld_amount ?
		if duration_in_s == 0 {
			result=MultiArg2((sc_error!("duration cannot be null"),0));
			return result;
		}

		// Compute instance deadline based on current time & duration parameter
		let deadline = self.blockchain().get_block_timestamp() + duration_in_s;	

		// Compute prize & fees base on egld_amount & fee 
		let fees: Self::BigUint = egld_amount.clone() * self.get_fee() / Self::BigUint::from(100 as u32);
		prize = egld_amount - fees.clone();

		// Compute next iid
		let new_iid = self.get_iid_cpt() + 1;

		// Fill owner information
		//let owner_info = OwnerInfo {
		//	pseudo: pseudo,
		//	url: url,
		//	picture_link: picture_link,
		//	free_text: free_text,
		//};

		// Fill instance information
		let instance_info = InstanceInfo {
			owner_address: caller,
			prize: prize,
			nb_players: 0u32,
			//owner_info: owner_info,
			deadline: deadline,
			winner_address: Address::zero(),
			claimed_status: false,
		};

		// Record new instance
		self.set_instance_info(new_iid, &instance_info);
		self.set_iid_cpt(new_iid);

		// Pay sc_owner with fee
		self.send().direct_egld(&self.blockchain().get_owner_address(), &fees, b"new instance fee");

		// Format result
		result=MultiArg2((Ok(()),new_iid));
		return result;
	}

	#[endpoint(trigger)]
	fn trigger(&self, iid: u32) -> SCResult<()>  {
		require!(self.get_instance_status(iid) == InstanceStatus::Ended, "Instance is not in the good state");

		// Get instance info
		let mut instance_info = self.get_instance_info(iid);

		// Get players of instance
		let instance_players = self.get_instance_players(iid); 

		if instance_players.len() == 0 {
			// No player, give prize back to instance owner
			instance_info.winner_address = instance_info.owner_address.clone();
		}
		else {
			// Choose winner 
			let seed = self.blockchain().get_block_random_seed();
			let mut rand = Random::new(*seed);
			let winning_address_index = rand.next() as usize % instance_players.len();
			instance_info.winner_address = instance_players[winning_address_index].clone();
		}

		// Record winner address
		self.set_instance_info(iid, &instance_info);

		Ok(())
	}

	/////////////////////////////////////////////////////////////////////
	// DApp endpoints : player API
	/////////////////////////////////////////////////////////////////////
	#[endpoint(play)]
	fn play(&self, iid: u32) -> SCResult<()>  {
		require!(self.get_instance_status(iid) == InstanceStatus::Running, "Instance is not active");
		require!(self.has_already_played(iid) == true, "Player has already played");

		// Add caller address to participants for this instance
		let mut instance_players = self.get_instance_players(iid);
		instance_players.push(self.blockchain().get_caller());
		self.set_instance_players(iid, &instance_players);

		// Increment number of players for this instance
		let mut instance_info = self.get_instance_info(iid);		
		instance_info.nb_players += 1; 
		self.set_instance_info(iid, &instance_info);

		Ok(())
	}

	#[endpoint(claim)]
	fn claim(&self, iid: u32) -> SCResult<()>  {
		// TODO
		Ok(())
	}

	/////////////////////////////////////////////////////////////////////
	// DApp view API
	/////////////////////////////////////////////////////////////////////
	#[view(getInstanceCounter)]
	#[storage_get("InstanceCounter")]
	fn get_iid_cpt(&self) -> u32;

	#[view(getInstanceInfo)]
	#[storage_get("InstancesInfo")]
	fn get_instance_info(&self, iid: u32) -> InstanceInfo<Self::BigUint>;

	#[view(getPlayStatus)]
	fn has_already_played(&self, iid: u32) -> bool {
		// TODO
		true
	}

	#[view(getRemainingTime)]
	fn get_remaining_time(&self, iid: u32) -> u64 {
		// Retrieve instance information
		let instance_info = self.get_instance_info(iid);
		let current_date_time = self.blockchain().get_block_timestamp();
		let mut remaing_time: u64 = 0;

		if instance_info.deadline > current_date_time {
			remaing_time = instance_info.deadline - current_date_time;
		}

		return remaing_time;
	}

	#[view(getInstanceStatus)]
	fn get_instance_status(&self, iid: u32) -> InstanceStatus {
		// Check instance ID exists
		// TODO : quid apres appel de cleanClaimedInstances ?
		let highest_iid = self.get_iid_cpt();

		if iid > highest_iid {
			return InstanceStatus::NotExisting;
		}
		else {
			// Retrieve instance information
			let instance_info = self.get_instance_info(iid);

			// Compute instance status based on fields values
			if instance_info.claimed_status == true {
				return InstanceStatus::Claimed;
			}
			else {
				if instance_info.winner_address != Address::zero() {
					return InstanceStatus::Triggered;
				}
				else {
					if self.blockchain().get_block_timestamp() > instance_info.deadline {
						return InstanceStatus::Ended;
					}
					else {
						return InstanceStatus::Running;
					}
				}
			}
		}		
	}

	#[view(getNbInstances)]
	fn get_nb_instances(&self) -> MultiResult2<u32, u32> {
		// TODO : quid apres appel de cleanClaimedInstances ?
		let highest_iid = self.get_iid_cpt();
		let mut nb_active_instances: u32 = 0;
		let mut nb_inactive_instances: u32 = 0;

		for i in 1..=highest_iid {
			if self.get_instance_status(i) == InstanceStatus::Running {
				nb_active_instances+=1;
			}
			else {
				nb_inactive_instances+=1;
			}
		}

		let result=MultiArg2((nb_active_instances,nb_inactive_instances));
		return result;	
	}

	#[view(getActiveInstances)]
	fn get_active_instances(&self) -> Vec<u32> {
		// TODO : quid apres appel de cleanClaimedInstances ?
		let highest_iid = self.get_iid_cpt();
		let mut active_instances = Vec::new();

		for i in 1..=highest_iid {
			if self.get_instance_status(i) == InstanceStatus::Running {
				active_instances.push(i);
			}
		}

		return active_instances;
	}

	#[view(getInactiveInstances)]
	fn get_inactive_instances(&self) -> Vec<u32> {
		// TODO : quid apres appel de cleanClaimedInstances ?
		let highest_iid = self.get_iid_cpt();
		let mut inactive_instances = Vec::new();

		for i in 1..=highest_iid {
			if self.get_instance_status(i) != InstanceStatus::Running {
				inactive_instances.push(i);
			}
		}

		return inactive_instances;
	}

	//TODO : does not work !!!
	#[view(getOwnerInstances)]
	fn get_owner_instances(&self) -> Vec<u32>  {
		// TODO : quid apres appel de cleanClaimedInstances ?
		let highest_iid = self.get_iid_cpt();
		let mut owner_instances = Vec::new();

		for i in 1..=highest_iid {
			// Retrieve instance information
			let instance_info = self.get_instance_info(i);

			if instance_info.owner_address == self.blockchain().get_caller() {
				owner_instances.push(i);
			}
		}

		return owner_instances;
	}

	/////////////////////////////////////////////////////////////////////
	// Local functions
	/////////////////////////////////////////////////////////////////////
	
	// InstancesInfo
	#[storage_set("InstancesInfo")]
	fn set_instance_info(&self, iid: u32, instance_info: &InstanceInfo<Self::BigUint>);

	#[storage_clear("InstancesInfo")]
	fn clear_instance_info(&self, iid: u32);

	// InstanceCounter
	#[storage_set("InstanceCounter")]
	fn set_iid_cpt(&self, instance_counter: u32);

	// InstancePlayers
	#[storage_set("InstancePlayers")]
	fn set_instance_players(&self, iid: u32, players: &Vec<Address>);

	#[storage_clear("InstancePlayers")]
	fn clear_instance_players(&self, iid: u32);

	#[storage_get("InstancePlayers")]
	fn get_instance_players(&self, iid: u32) -> Vec<Address>;

	// Fee
	#[storage_set("Fee")]
	fn set_fee(&self, fee_percent: Self::BigUint);
}
