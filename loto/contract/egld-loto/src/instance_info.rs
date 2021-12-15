
use elrond_wasm::api::ManagedTypeApi;
use elrond_wasm::types::BigUint;
use elrond_wasm::types::ManagedAddress;
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
pub struct InstanceInfo <M: ManagedTypeApi>{
	pub sponsor_address: ManagedAddress<M>,
	pub prize: BigUint<M>,
	//pub sponsor_info: SponsorInfo,
	pub deadline: u64,
	pub winner_address: ManagedAddress<M>,
	pub claimed_status: bool,
}
