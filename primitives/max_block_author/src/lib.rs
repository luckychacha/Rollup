
#[derive(Encode, Decode, sp_core::RuntimeDebug, Clone, PartialEq, TypeInfo)]
pub struct MaxBlockAuthorInherentDataProvider {
	times: u32,
}
pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"maxblockauthor";

impl MaxBlockAuthorInherentDataProvider {
	fn new(times: u32) -> Self {
		Self { times }
	}
}