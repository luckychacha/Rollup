
use codec::{Encode, Decode};
use scale_info::TypeInfo;
use sp_inherents::InherentIdentifier;


#[derive(Encode, Decode, sp_core::RuntimeDebug, Clone, PartialEq, TypeInfo)]
pub struct MaxBlockAuthorInherentDataProvider {
	times: u32,
}
pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"maxblcka";

impl MaxBlockAuthorInherentDataProvider {
	fn new(times: u32) -> Self {
		Self { times }
	}
}