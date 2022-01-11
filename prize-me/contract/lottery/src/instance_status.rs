
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Clone, Copy)]
pub enum InstanceStatus {
	NotExisting,
	Running,
	Ended,
	Triggered,
	Claimed,
}
