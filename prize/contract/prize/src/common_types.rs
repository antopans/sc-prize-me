use elrond_wasm::api::ManagedTypeApi;
use elrond_wasm::types::BigUint;
use elrond_wasm::types::ManagedAddress;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm::types::TokenIdentifier;

extern crate variant_count;
use variant_count::VariantCount;

elrond_wasm::derive_imports!();

////////////////////////////////////////////////////////////////////
// Enums
////////////////////////////////////////////////////////////////////

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, PartialEq, Clone, Copy, VariantCount, Ord, PartialOrd, Eq)]
pub enum PrizeType {
    EgldPrize,
    EsdtPrize,
    UnknownPrize,
}

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Clone, Copy, VariantCount, Ord, PartialOrd, Eq)]
pub enum InstanceStatus {
    NotExisting,
    Running,
    Ended,
    Triggered,
    Claimed,
}

////////////////////////////////////////////////////////////////////
// Structures
////////////////////////////////////////////////////////////////////

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct SponsorInfo<M: ManagedTypeApi> {
    pub pseudo: ManagedBuffer<M>,
    pub url: ManagedBuffer<M>,
    pub logo_link: ManagedBuffer<M>,
    pub free_text: ManagedBuffer<M>,
    pub reward_percent: u8,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct PrizeInfo<M: ManagedTypeApi> {
    pub prize_type: PrizeType,
    pub token_identifier: TokenIdentifier<M>,
    pub token_nonce: u64,
    pub token_amount: BigUint<M>,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct InstanceInfo<M: ManagedTypeApi> {
    pub sponsor_address: ManagedAddress<M>,
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

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct FeePolicy<M: ManagedTypeApi> {
    pub fee_amount_egld: BigUint<M>,
    pub sponsor_reward_percent: u8,
}

