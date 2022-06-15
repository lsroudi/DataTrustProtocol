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

pub type VerificationMethods<T> = BoundedBTreeMap<MethodTypeOf<T>, PublicKeyDetails<T>, <T as Config>::PublicKeysPerDid>;

pub type VerificationRelationShips<T> = BoundedBTreeMap<MethodTypeOf<T>, PublicKeyDetails<T>, <T as Config>::PublicKeysPerDid>;

pub type AuthenticationSet<T> = BoundedBTreeSet<MethodTypeOf<T>, <T as Config>::PublicKeysPerDid>;
pub type AssertionSet<T> = BoundedBTreeSet<MethodTypeOf<T>, <T as Config>::PublicKeysPerDid>;

#[derive(Clone, Decode, Encode, PartialEq, TypeInfo,RuntimeDebug, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]

pub struct DidProperties<T: Config> {
	// the submitter of Did Creation
	pub submitter: AccountIdOf<T>,
	// the identifier of DID, refer to the following link for more information
	// https://www.w3.org/TR/did-core/#identifiers
	pub did: IdentifierOf<T>,
	// The verification methods regarding the W3 specification 
	// For more information please refer to the following link 
	// https://www.w3.org/TR/did-core/#verification-methods
	pub verification_method: VerificationMethods<T>,
	// The authentication rleation ship method regarding the W3  
	// specification. For more information please refer to the following link 
	// https://www.w3.org/TR/did-core/#authentication
	pub authentication: Option<DidPublicKey>,
	// The assertion relation ship methods regarding the W3 specification. 
	// For more information please refer to the following link 
	// https://www.w3.org/TR/did-core/#assertion
	pub assertion: Option<DidPublicKey>,
	// The verification relation ships expresses the relationship between
	// the DID subject and a verification method.  
	// For more information please refer to the following link 
	// https://www.w3.org/TR/did-core/#verification-relationships	
	pub verification_relation_ships: VerificationRelationShips<T>

}

impl<T:Config>  DidProperties<T> {

	pub fn new(){

	}

}