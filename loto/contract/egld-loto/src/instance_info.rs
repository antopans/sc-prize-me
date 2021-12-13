
use elrond_wasm::api::{BigUintApi};
use elrond_wasm::types::Address;
use elrond_wasm::String;

elrond_wasm::derive_imports!();

//mod owner_info;
//use owner_info::OwnerInfo;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct OwnerInfo {
	pub pseudo: String,
	pub url: String,
	pub picture_link: String,
	pub free_text: String,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct InstanceInfo<BigUint: BigUintApi> {
	pub owner_address: Address,
	pub prize: BigUint,
	pub nb_players: u32,
	//pub owner_info: OwnerInfo,
	pub deadline: u64,
	pub winner_address: Address,
	pub claimed_status: bool,
}
