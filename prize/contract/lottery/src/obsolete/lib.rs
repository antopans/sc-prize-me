#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(clippy::too_many_arguments)]

elrond_wasm::imports!();


mod loto_info;
mod random;
mod status;

use loto_info::LotoInfo;
use random::Random;
use status::Status;


#[elrond_wasm_derive::contract]

pub trait Loto {
	
	#[init]
	fn init(&self) {
	}

	#[endpoint]
	fn start(&self, 
		ticket_price: Self::BigUint, delay_in_s: u64, fee_percent: Self::BigUint) -> SCResult<()> {
		only_owner!(self, "Caller address not allowed");
		require!(self.status() == Status::Inactive, "Loto is already started!");
		require!(ticket_price > 0, "Ticket price must be higher than 0!");
		require!(fee_percent<=100, "fee_percent can't be higher than 100!");
		require!(delay_in_s > 0, "delay must be higher than 0!");

		let timestamp = self.blockchain().get_block_timestamp();	
		let deadline = timestamp + delay_in_s;

		let info = LotoInfo {
			ticket_price,
			deadline,
			current_ticket_number: 0u32,
			fee_pool: Self::BigUint::zero(),
			prize_pool: Self::BigUint::zero(),
			fee_percent,
		};

		self.set_loto_info(&info);

		Ok(())
	}
	
	#[endpoint]
	#[payable("EGLD")]
	fn buy_ticket(&self, #[payment] payment: Self::BigUint) -> SCResult<()> {
		match self.status() {
			Status::Inactive => sc_error!("Ticket sales are not open."),
			Status::Running => self.update_after_buy_ticket(&payment),
			Status::Ended => {sc_error!("Ticket sales are closed. Awaiting winner announcement.")
			},
		}
	}

	#[endpoint]
	fn get_fee(&self) -> SCResult<()> {
		only_owner!(self, "Caller address not allowed");
		match self.status() {
			Status::Inactive => sc_error!("Loto is inactive!"),
			Status::Running | Status::Ended => {
				self.pay_current_fee();
				Ok(())
			},
		}
	}
	
	#[endpoint]
	fn cancel(&self) -> SCResult<()> {
		only_owner!(self, "Caller address not allowed");
		match self.status() {
			Status::Inactive => sc_error!("Loto is inactive!"),
			Status::Running | Status::Ended => {
				self.refund_buyers();
				self.pay_current_fee();
				self.clear_storage();
				Ok(())
			},
		}
	}

	#[view(status)]
	fn status(&self) -> Status {
		if self.is_empty_loto_info() {
			return Status::Inactive;
		}

		let info = self.get_loto_info();

		if self.blockchain().get_block_timestamp() > info.deadline {
			return Status::Ended;
		}

		Status::Running
	}

	#[view(test2)]
	fn test2(&self, val: u32) -> u32 {
		//only_owner!(self, "Caller address not allowed");
		val+1		
	}
	
	#[endpoint]
	fn test(&self, val: u32) -> u32 {
		//only_owner!(self, "Caller address not allowed");
		val+1
		
	}
	
	#[endpoint]
	fn test3(&self, val: u32) -> MultiResult2<SCResult<()>, u32> {
		//only_owner!(self, "Caller address not allowed");
		let res=MultiArg2((sc_error!("test3 error"),val+7));
		res	
	}
	
	#[view(test4)]
	fn test4(&self, val: u32) -> MultiResult2<SCResult<()>, u32> {
		//only_owner!(self, "Caller address not allowed");
		let res=MultiArg2((Ok(()),val+3));
		res	
	}
	
	
	#[endpoint]
	fn trigger(&self) -> SCResult<()> {
		only_owner!(self, "Caller address not allowed");
		match self.status() {
			Status::Inactive => sc_error!("Loto is inactive!"),
			Status::Running => sc_error!("Loto is still running!"),
			Status::Ended => {
				self.distribute_prizes();
				self.pay_current_fee();
				self.clear_storage();
				Ok(())
			},
		}
	}

	fn update_after_buy_ticket(&self, payment: &Self::BigUint) -> SCResult<()> {
		let mut info = self.get_loto_info();
		let caller = self.blockchain().get_caller();

		require!(payment == &info.ticket_price, "Wrong ticket price!");

		self.set_ticket_holder(info.current_ticket_number, &caller);
		self.set_holder_number_of_tickets(&caller, self.get_holder_number_of_tickets(&caller) + 1);
		self.set_holder_last_ticket_number(&caller, info.current_ticket_number);
		info.current_ticket_number += 1;

		let fee: Self::BigUint = info.ticket_price.clone() * info.fee_percent.clone() / Self::BigUint::from(100 as u32);

		info.prize_pool += info.ticket_price.clone() - fee.clone();
		info.fee_pool += fee.clone();

		self.set_loto_info(&info);

		Ok(())
	}

	fn distribute_prizes(&self) {
		let mut info = self.get_loto_info();
		let total_tickets = info.current_ticket_number;

		if info.current_ticket_number > 0 {
			let prize: Self::BigUint;
			prize = info.prize_pool.clone();

			// Pay winner 
			let seed = self.blockchain().get_block_random_seed();
			let mut rand = Random::new(*seed);
			let winning_ticket_id: u32;
			winning_ticket_id = rand.next() % total_tickets;

			let winner_address = self.get_ticket_holder(winning_ticket_id);
			
			self.send().direct_egld(&winner_address, &prize, b"You have the winning ticket! Congratulations!");
			info.prize_pool -= prize;
		}

		self.set_loto_info(&info);
	}

	fn pay_current_fee(&self) {
		let mut info = self.get_loto_info();

		if info.fee_pool > 0 {
			let fee: Self::BigUint;
			fee = info.fee_pool.clone();

			// Pay owner with fee
			self.send().direct_egld(&self.blockchain().get_owner_address(), &fee, b"loto fee");

			info.fee_pool -= fee;
		}

		self.set_loto_info(&info);
	}

	fn refund_buyers(&self) {
		let info = self.get_loto_info();

		let fee: Self::BigUint = info.ticket_price.clone() * info.fee_percent.clone() / Self::BigUint::from(100 as u32);
		let ticket_value: Self::BigUint = info.ticket_price.clone() - fee.clone();

		for i in 0..info.current_ticket_number {
			let refund_address = self.get_ticket_holder(i);
			let holder_last_ticket_number = self.get_holder_last_ticket_number(&refund_address);
			let holder_number_of_tickets = self.get_holder_number_of_tickets(&refund_address);
			let refund = ticket_value.clone() * Self::BigUint::from(holder_number_of_tickets as u32);

			if i == holder_last_ticket_number {
				self.send().direct_egld(&refund_address, &refund, b"Loto cancelled, this is your ticket refund!");
			}
		}

		self.set_loto_info(&info);
	}

	fn clear_storage(&self) {
		let info = self.get_loto_info();

		for i in 0..info.current_ticket_number {
			self.clear_holder_last_ticket_number(&self.get_ticket_holder(i));
			self.clear_holder_number_of_tickets(&self.get_ticket_holder(i));
			self.clear_ticket_holder(i);
		}

		self.clear_loto_info();
	}

	#[storage_set("lotoInfo")]
	fn set_loto_info(&self, loto_info: &LotoInfo<Self::BigUint>);

	#[view(lotoInfo)]
	#[storage_get("lotoInfo")]
	fn get_loto_info(&self) -> LotoInfo<Self::BigUint>;

	#[storage_is_empty("lotoInfo")]
	fn is_empty_loto_info(&self) -> bool;

	#[storage_clear("lotoInfo")]
	fn clear_loto_info(&self);

	#[storage_set("ticketHolder")]
	fn set_ticket_holder(&self, ticket_id: u32, ticket_holder: &Address);

	#[storage_get("ticketHolder")]
	fn get_ticket_holder(&self, ticket_id: u32) -> Address;

	#[storage_clear("ticketHolder")]
	fn clear_ticket_holder(&self, ticket_id: u32);

	#[storage_set("holderLastTicketNumber")]
	fn set_holder_last_ticket_number(&self, ticket_holder: &Address, ticket_id: u32);

	#[storage_get("holderLastTicketNumber")]
	fn get_holder_last_ticket_number(&self, ticket_holder: &Address) -> u32;

	#[storage_clear("holderLastTicketNumber")]
	fn clear_holder_last_ticket_number(&self, ticket_holder: &Address);

	#[storage_set("holderNumberOfTickets")]
	fn set_holder_number_of_tickets(&self, ticket_holder: &Address, number_of_ticket: u32);

	#[storage_get("holderNumberOfTickets")]
	fn get_holder_number_of_tickets(&self, ticket_holder: &Address) -> u32;

	#[storage_clear("holderNumberOfTickets")]
	fn clear_holder_number_of_tickets(&self, ticket_holder: &Address);	
}
