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

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, PartialEq, Clone, Copy, VariantCount, Ord, PartialOrd, Eq)]
pub enum InstanceStatus {
    NotExisting,
    Running,
    Ended,
    Triggered,
    Claimed,
    Disabled,
}

////////////////////////////////////////////////////////////////////
// Structures
////////////////////////////////////////////////////////////////////

// Information filled at instance creation
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
    pub prize_type: PrizeType,
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

// State of instance, content depends on instance lifecycle
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct WinnerInfo<M: ManagedTypeApi> {
    pub ticket_number: usize,
    pub address: ManagedAddress<M>,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct InstanceState<M: ManagedTypeApi> {
    pub sponsor_rewards_pool: BigUint<M>,
    pub claimed_status: bool,
    pub winner_info: WinnerInfo<M>,
    pub disabled: bool,
}

// data format for endpoint return
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct GetInfoStruct<M: ManagedTypeApi> {
    pub iid: u32,
    pub instance_status: InstanceStatus,
    pub number_of_players: usize,
    pub winner_info: WinnerInfo<M>,
    pub sponsor_info: SponsorInfo<M>,
    pub prize_info: PrizeInfo<M>,
    pub deadline: u64,
}
