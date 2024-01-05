use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_inherents::InherentIdentifier;

#[derive(Encode, Decode, sp_core::RuntimeDebug, Clone, PartialEq, TypeInfo)]
pub struct MaxBlockAuthorInherentDataProvider {
	pub times: u32,
	pub awaiting_inherent_processing: bool,
}
pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"maxblcka";

impl MaxBlockAuthorInherentDataProvider {
	fn new(times: u32, awaiting_inherent_processing: bool) -> Self {
		Self { times, awaiting_inherent_processing }
	}
}

#[cfg(feature = "std")]
#[async_trait::async_trait]
impl sp_inherents::InherentDataProvider for MaxBlockAuthorInherentDataProvider {
	async fn provide_inherent_data(
		&self,
		inherent_data: &mut sp_inherents::InherentData,
	) -> Result<(), sp_inherents::Error> {
		inherent_data.put_data(INHERENT_IDENTIFIER, &self)
	}

	async fn try_handle_error(
		&self,
		_: &InherentIdentifier,
		_: &[u8],
	) -> Option<Result<(), sp_inherents::Error>> {
		None
	}
}
