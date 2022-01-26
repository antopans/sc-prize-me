
use elrond_wasm::api::{BigUintApi};

elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct LotoInfo<BigUint: BigUintApi> {
	pub ticket_price: BigUint,
	pub deadline: u64,
	pub current_ticket_number: u32,
	pub prize_pool: BigUint,
	pub fee_pool: BigUint,
    pub fee_percent : BigUint,
}
