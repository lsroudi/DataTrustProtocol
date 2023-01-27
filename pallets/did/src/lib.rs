#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use sp_std::{boxed::Box, fmt::Debug, prelude::Clone};
use frame_support::{traits::Get};
use sp_core::{ecdsa, ed25519, sr25519};
use codec::Codec;
#[frame_support::pallet]
pub mod pallet {
	use super::*;
    use frame_support::{pallet_prelude::*, error::BadOrigin};
    use frame_system::pallet_prelude::*;

	use crate::{did_attributes::{DidProperties,DidPublicKey}};
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn get_did)]
	pub type Did<T> = StorageMap<_, Blake2_128Concat, DidIdentifierOf<T>, Details<T>>;

	pub type DidIdentifierOf<T> = <T as Config>::DidIdentifier;
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    #[pallet::config]
    pub trait Config: frame_system::Config + Debug {
		type DidIdentifier: Parameter + MaxEncodedLen;
		

		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		#[pallet::constant]
		type PublicKeysPerDid: Get<u32>;

    }

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		DidStored(u32, T::AccountId),
	}

	#[derive(Clone, Decode, RuntimeDebug, Encode, Eq, PartialEq, TypeInfo)]
	pub enum Signature {
		Ed25519(ed25519::Signature),
		Sr25519(sr25519::Signature),
		Ecdsa(ecdsa::Signature),
	}

	#[derive(Clone, Decode, Encode, PartialEq, TypeInfo, MaxEncodedLen,Debug)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct Details<T: Config> {
		/// a unique identifier
		pub did: DidIdentifierOf<T>,
		/// who can create a did
		pub submitter: AccountIdOf<T>,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn did_create(origin: OriginFor<T>, signature : Signature, details:Box<Details<T>>) -> DispatchResult {

			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;


			ensure!(who == details.submitter, BadOrigin);

			let did = details.did.clone();


			// TODO ==> check if the AccountIdOf can pay for this transaction
			// TODO ==> Validation pre insertion


			// Emit an event.

			Self::deposit_event(Event::DidStored(1, who));

			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}


