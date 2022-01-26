use elrond_wasm::api::ManagedTypeApi;
use elrond_wasm::types::BigUint;
use elrond_wasm::types::ManagedAddress;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm::types::TokenIdentifier;

elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct SponsorInfo<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub pseudo: ManagedBuffer<M>,
    pub url: ManagedBuffer<M>,
    pub logo_link: ManagedBuffer<M>,
    pub free_text: ManagedBuffer<M>,
    pub reward_percent: u8,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct PrizeInfo<M: ManagedTypeApi> {
    pub token_identifier: TokenIdentifier<M>,
    pub token_nonce: u64,
    pub token_amount: BigUint<M>,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct InstanceInfo<M: ManagedTypeApi> {
    pub sponsor_info: SponsorInfo<M>,
    pub prize_info: PrizeInfo<M>,
    pub deadline: u64,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct InstanceData<M: ManagedTypeApi> {
    pub sponsor_rewards_pool: BigUint<M>,
    pub claimed_status: bool,
    pub winner_address: ManagedAddress<M>,
    pub disabled: bool,
}

