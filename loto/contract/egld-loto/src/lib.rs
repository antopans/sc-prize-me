#![no_std]

elrond_wasm::imports!();

mod instance_info;
mod instance_status;
mod random;

use instance_info::SponsorInfo;
use instance_info::InstanceInfo;
use instance_status::InstanceStatus;
use random::Random;

#[elrond_wasm::contract]
pub trait Loto {

	/////////////////////////////////////////////////////////////////////
	// SC Management API
	/////////////////////////////////////////////////////////////////////
	
	#[init]
	fn init(&self) -> SCResult<()> {
		self.iid_counter_mapper().set_if_empty(&0u32);
		Ok(())
	}

	/////////////////////////////////////////////////////////////////////
	// Administrator API
	/////////////////////////////////////////////////////////////////////

	#[endpoint(triggerEnded)]
	fn trigger_ended_instances(&self) -> SCResult<()> {
		only_owner!(self, "Caller address not allowed");
		let ended_instances: Vec<u32> = self.get_instance_ids(InstanceStatus::Ended);

		for iid in ended_instances.iter() {
			self.trigger(iid.clone());
		}

		Ok(())
	}

	#[endpoint(cleanClaimed)]
	fn clean_claimed_instances(&self) -> SCResult<()> {
		only_owner!(self, "Caller address not allowed");
		let claimed_instances: Vec<u32> = self.get_instance_ids(InstanceStatus::Claimed);

		for iid in claimed_instances.iter() {
			self.instance_players_set_mapper(iid.clone()).clear();
			self.instance_players_vec_mapper(iid.clone()).clear();
			self.instance_info_mapper().remove(&iid);
		}

		Ok(())
	}

	/////////////////////////////////////////////////////////////////////
	// DApp endpoints : sponsor API
	/////////////////////////////////////////////////////////////////////
	#[payable("EGLD")]
	#[endpoint(create)]
	fn create_instance(&self, 
		#[payment] egld_amount: BigUint, 
		duration_in_s: u64, 
		pseudo: ManagedBuffer, 
		url: ManagedBuffer, 
		picture_link: ManagedBuffer, 
		free_text: ManagedBuffer) -> MultiResult2<SCResult<()>, OptionalResult<u32>>  {
		
		let result;
		
		// Check validity of parameters 
		if duration_in_s == 0 {
			result=MultiArg2((sc_error!("duration cannot be null"), OptionalResult::None));
			return result;
		}

		// Compute instance deadline based on current time & duration parameter
		let deadline = self.blockchain().get_block_timestamp() + duration_in_s;	

		// Compute next iid
		let new_iid = self.iid_counter_mapper().get() + 1;

		// Fill sponsor information
		let sponsor_info = SponsorInfo {
			pseudo: pseudo,
			url: url,
			picture_link: picture_link,
			free_text: free_text,
		};

		// Fill instance information
		let instance_info = InstanceInfo {
			sponsor_address: self.blockchain().get_caller(),
			prize: egld_amount,
			sponsor_info: sponsor_info,
			deadline: deadline,
			winner_address: ManagedAddress::zero(),
			claimed_status: false,
		};

		// Record new instance
		self.instance_info_mapper().insert(new_iid, instance_info);
		self.iid_counter_mapper().set(&new_iid);

		// Format result
		result=MultiArg2((Ok(()),OptionalResult::Some(new_iid)));
		return result;
	}

	#[endpoint(trigger)]
	fn trigger(&self, iid: u32) -> SCResult<()>  {
		require!(self.get_instance_status(iid) == InstanceStatus::Ended, "Instance is not in the good state");

		// Get instance info
		match self.instance_info_mapper().get(&iid) {
			None => return sc_error!("Unexpected error"),
			Some(instance_info) => {
				if (self.blockchain().get_caller() != instance_info.sponsor_address) && 
				   (self.blockchain().get_caller() != self.blockchain().get_owner_address()) {
					return sc_error!("Instance can only be triggered by its sponsor or by administrator");
				}

				let mut updated_instance_info = instance_info;

				if self.instance_players_vec_mapper(iid).len() == 0 {
					// No player, give prize back to instance sponsor
					updated_instance_info.winner_address = updated_instance_info.sponsor_address.clone();
				}
				else {
					// Choose winner 
					let seed = self.blockchain().get_block_random_seed_legacy();
					let mut rand = Random::new(*seed);
					let winning_address_index = (rand.next() as usize % self.instance_players_vec_mapper(iid).len()) + 1;
					updated_instance_info.winner_address = self.instance_players_vec_mapper(iid).get(winning_address_index);
				}

				// Record winner address
				self.instance_info_mapper().insert(iid, updated_instance_info);
			},
		}

		Ok(())
	}

	/////////////////////////////////////////////////////////////////////
	// DApp endpoints : player API
	/////////////////////////////////////////////////////////////////////
	#[endpoint(play)]
	fn play(&self, iid: u32) -> SCResult<()>  {
		let caller = self.blockchain().get_caller();
		require!(self.get_instance_status(iid) == InstanceStatus::Running, "Instance is not active");
		require!(self.instance_players_set_mapper(iid).contains(&caller) == false, "Player has already played");
	
		// Add caller address to participants for this instance
		self.instance_players_set_mapper(iid).insert(caller.clone());
		self.instance_players_vec_mapper(iid).push(&caller);

		Ok(())
	}

	#[endpoint(claim)]
	fn claim(&self, iid: u32) -> SCResult<()>  {
		require!(self.get_instance_status(iid) == InstanceStatus::Triggered, "Instance is not in the good state");

		// Get instance info
		match self.instance_info_mapper().get(&iid) {
			None => return sc_error!("Unexpected error"),
			Some(instance_info) => {

				// Check caller is the winner
				if instance_info.winner_address != self.blockchain().get_caller() {
					return sc_error!("Prize can only be claimed by the winner");
				}

				// Send prize to winner address
				self.send().direct_egld(&instance_info.winner_address, &instance_info.prize, b"Prize claimed");

				// Update claimed status
				let mut updated_instance_info = instance_info;
				updated_instance_info.claimed_status = true;
				self.instance_info_mapper().insert(iid, updated_instance_info);
			},
		}

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
		// Retrieve instance information
		let mapper_value = self.instance_info_mapper().get(&iid);

		match mapper_value {
			None => return InstanceStatus::NotExisting,
			Some(instance_info) => {
				// Compute instance status based on fields values
				if instance_info.claimed_status == true {
					return InstanceStatus::Claimed;
				}
				else {
					if instance_info.winner_address != ManagedAddress::zero() {
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
			},
		}
	}

	#[view(getInfo)]
	fn get_instance_info(&self, iid: u32) -> MultiResult4<
		SCResult<()>, 
		OptionalResult<InstanceInfo<Self::Api>>, 
		OptionalResult<InstanceStatus>, 
		OptionalResult<usize>>  {	

		let result: MultiArg4<SCResult<()>, OptionalResult<InstanceInfo<Self::Api>>, OptionalResult<InstanceStatus>, OptionalResult<usize>>;

		// Retrieve instance information
		match self.instance_info_mapper().get(&iid) {
			None => {
				// Instance does not exist
				result=MultiArg4((sc_error!("Instance does not exists"), OptionalArg::None, OptionalArg::None, OptionalArg::None));
			},
			Some(instance_info) => {
				// Instance found : return info, status and number of players
				result=MultiArg4((Ok(()), OptionalArg::Some(instance_info), OptionalArg::Some(self.get_instance_status(iid)), OptionalArg::Some(self.instance_players_set_mapper(iid).len())));
			},
		}

		return result;
	}

	#[view(getRemainingTime)]
	fn get_remaining_time(&self, iid: u32) -> MultiResult2<SCResult<()>, OptionalResult<u64>>  {
		let result;

		// Retrieve instance information
		match self.instance_info_mapper().get(&iid) {
			None => {
				// Instance does not exist
				result=MultiArg2((sc_error!("Instance does not exists"), OptionalResult::None));
			},
			Some(instance_info) => {
				let current_date_time = self.blockchain().get_block_timestamp();
				let mut remaing_time: u64 = 0;

				if instance_info.deadline > current_date_time {
					remaing_time = instance_info.deadline - current_date_time;
				}

				result=MultiArg2((Ok(()), OptionalResult::Some(remaing_time)));
			},
		}

		return result;
	}


	#[view(hasStatus)]
	fn is_instance_with_status(&self, instance_status: InstanceStatus) -> bool {
		let instances: Vec<u32> = self.get_instance_ids(instance_status);
		return instances.len() != 0;
	}

	#[view(getIDs)]
	fn get_instance_ids(&self, instance_status: InstanceStatus) -> Vec<u32> {
		let mut instance_ids = Vec::new();

		// Return all instances IDs which meet the status filter provided in parameter
		for iid in self.instance_info_mapper().keys() {
			if self.get_instance_status(iid) == instance_status {
				instance_ids.push(iid.clone());
			}
		}

		return instance_ids;
	}

	#[view(getSponsorIDs)]
	fn get_sponsor_instances(&self, sponsor_address: ManagedAddress) -> Vec<u32> {
		let mut sponsor_iids = Vec::new();

		// Return all instances IDs with sponsor address matching the one provided in parameter
		for instance in self.instance_info_mapper().iter() {
			if instance.1.sponsor_address.clone() == sponsor_address {
				sponsor_iids.push(instance.0);
			}
		}

		return sponsor_iids;
	}

	#[view(getPlayerIDs)]
	fn get_player_instances(&self, player_address: ManagedAddress) -> Vec<u32> {
		let mut player_iids = Vec::new();

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
	fn has_won(&self, iid: u32, player_address: ManagedAddress) -> MultiResult2<SCResult<()>, OptionalResult<bool>>  {

		// Retrieve instance information
		match self.instance_info_mapper().get(&iid) {

			None => {
				// Instance does not exist
				return MultiArg2((sc_error!("Instance does not exists"), OptionalResult::None));
			},
			Some(instance_info) => {
				// Return true is player_address provided in parameter is the winner address for the specified instance ID
				if instance_info.winner_address == player_address {
					return MultiArg2((Ok(()), OptionalResult::Some(true)));
				}
			},
		}

		return MultiArg2((Ok(()), OptionalResult::Some(false)));
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

	// Instance players
	#[storage_mapper("instance_players_set")]
	fn instance_players_set_mapper(&self, iid: u32) -> SetMapper<ManagedAddress>;
	#[storage_mapper("instance_players_vec")]
	fn instance_players_vec_mapper(&self, iid: u32) -> VecMapper<ManagedAddress>;
}