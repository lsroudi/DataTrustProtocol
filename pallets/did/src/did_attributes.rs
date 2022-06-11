use codec::{Decode, Encode, MaxEncodedLen, WrapperTypeEncode};
use scale_info::TypeInfo;
use sp_core::{ecdsa, ed25519, sr25519};
use crate::{
	Config,AccountIdOf,IdentifierOf,MethodTypeOf
};
use frame_support::{
	RuntimeDebug,
	storage::{bounded_btree_map::BoundedBTreeMap, bounded_btree_set::BoundedBTreeSet},
};

#[derive(Clone, Decode, RuntimeDebug, Encode, Eq, Ord, PartialEq, PartialOrd, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]

pub enum DidPublicKey {
	Ed25519(ed25519::Public),
	Sr25519(sr25519::Public),
	Ecdsa(ecdsa::Public),
}
#[derive(Clone, RuntimeDebug, Decode, Encode, PartialEq, Ord, PartialOrd, Eq, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]

pub struct PublicKeyDetails<T: Config> {
	/// A public key the DID controls.
	pub key: DidPublicKey,
	pub submitter: AccountIdOf<T>
	
}

pub type VerificationMethods<T> = BoundedBTreeMap<<T as frame_system::Config>::Hash, PublicKeyDetails<T>, <T as Config>::PublicKeysPerDid>;

#[derive(Clone, Decode, Encode, PartialEq, TypeInfo,RuntimeDebug, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]

pub struct DidProperties<T: Config> {
	// the submitter of Did Creation
	pub submitter: AccountIdOf<T>,
	// the identifier of DID
	pub did: IdentifierOf<T>,
	pub verification_method: VerificationMethods<T>,
	pub authentication: MethodTypeOf<T>,
	pub assertion: MethodTypeOf<T>,

}