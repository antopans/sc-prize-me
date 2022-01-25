use elrond_wasm::api::ManagedTypeApi;
use elrond_wasm::types::BigUint;

elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct FeePolicy<M: ManagedTypeApi> {
    pub fee_amount_egld: BigUint<M>,
    pub sponsor_reward_percent: u8,
}



