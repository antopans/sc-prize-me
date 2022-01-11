
use elrond_wasm::String;

elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct OwnerInfo {
	pub pseudo: String,
	pub url: String,
	pub picture_link: String,
	pub free_text: String,
}
