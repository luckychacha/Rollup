
use codec::{Encode, Decode};
use scale_info::TypeInfo;
use sp_inherents::InherentIdentifier;


#[derive(Encode, Decode, sp_core::RuntimeDebug, Clone, PartialEq, TypeInfo)]
pub struct MaxBlockAuthorInherentDataProvider {
	pub times: u32,
	pub pub awaiting_inherent_processing: bool,
}
pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"maxblcka";

impl MaxBlockAuthorInherentDataProvider {
	fn new(times: u32, awaiting_inherent_processing: bool) -> Self {
		Self { times, awaiting_inherent_processing }
	}
}