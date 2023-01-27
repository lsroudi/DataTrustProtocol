#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use sp_std::{boxed::Box, fmt::Debug, prelude::Clone};
use frame_support::{traits::Get};
use sp_core::{ecdsa, ed25519, sr25519};

#[frame_support::pallet]
pub mod pallet {
	use super::*;
    use codec::Codec;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + Debug {

		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		#[pallet::constant]
		type PublicKeysPerDid: Get<u32>;
		/// Type for a DID subject identifier.
		type Identifier: Parameter + MaxEncodedLen;
    }

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		DidStored(u32, T::AccountId),
	}

	#[derive(Clone, Decode, RuntimeDebug, Encode, Eq, PartialEq, TypeInfo)]
	pub enum DidSignature {
		Ed25519(ed25519::Signature),
		Sr25519(sr25519::Signature),
		Ecdsa(ecdsa::Signature),
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn did_create(origin: OriginFor<T>, signature : DidSignature) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			//ensure!(who == attributes.submitter, BadOrigin);

			//let did = attributes.did.clone();

			// TODO ==> check if the AccountIdOf can pay for this transaction
			// TODO ==> Validation pre insertion


			// Emit an event.
			Self::deposit_event(Event::DidStored(1, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}


