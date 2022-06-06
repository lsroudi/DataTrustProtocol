
use codec::{Decode, Encode, MaxEncodedLen, WrapperTypeEncode};
use scale_info::TypeInfo;
use crate::{
	Config,AccountIdOf
};
use frame_support::{
	RuntimeDebug,
};
#[derive(Clone, RuntimeDebug, Decode, Encode, PartialEq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct DidAttributes<T: Config> {
	pub submitter: AccountIdOf<T>,

}