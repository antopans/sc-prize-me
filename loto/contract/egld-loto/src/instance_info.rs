
use elrond_wasm::api::{BigUintApi};
use elrond_wasm::types::Address;
use elrond_wasm::String;

elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct SponsorInfo {
	pub pseudo: String,
	pub url: String,
	pub picture_link: String,
	pub free_text: String,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct InstanceInfo<BigUint: BigUintApi> {
	pub sponsor_address: Address,
	pub prize: BigUint,
	//pub nb_players: u32,
	//pub sponsor_info: SponsorInfo,
	pub deadline: u64,
	pub winner_address: Address,
	pub claimed_status: bool,
}
