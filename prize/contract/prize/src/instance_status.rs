elrond_wasm::derive_imports!();

extern crate variant_count;
use variant_count::VariantCount;

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Clone, Copy, VariantCount, Ord, PartialOrd, Eq)]
pub enum InstanceStatus {
    NotExisting,
    Running,
    Ended,
    Triggered,
    Claimed,
}
