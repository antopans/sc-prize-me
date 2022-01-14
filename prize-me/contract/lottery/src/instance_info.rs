use elrond_wasm::api::ManagedTypeApi;
use elrond_wasm::types::BigUint;
use elrond_wasm::types::ManagedAddress;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm::types::TokenIdentifier;

elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct SponsorInfo<M: ManagedTypeApi> {
    pub pseudo: ManagedBuffer<M>,
    pub url: ManagedBuffer<M>,
    pub picture_link: ManagedBuffer<M>,
    pub free_text: ManagedBuffer<M>,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct InstanceInfo<M: ManagedTypeApi> {
    pub sponsor_address: ManagedAddress<M>,
    pub token_identifier: TokenIdentifier<M>,
    pub token_amount: BigUint<M>,
    pub sponsor_info: SponsorInfo<M>,
    pub deadline: u64,
    pub winner_address: ManagedAddress<M>,
    pub claimed_status: bool,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct InstanceInfoTmp<M: ManagedTypeApi> {
    pub token_amount: BigUint<M>,
    pub sponsor_info: SponsorInfo<M>,
}