#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use sp_std::{boxed::Box, fmt::Debug, prelude::Clone};
use frame_support::{traits::Get};
use codec::Codec;
#[frame_support::pallet]
pub mod pallet {
	use super::*;
    use frame_support::{pallet_prelude::*, 
		error::BadOrigin
	};
    use frame_system::pallet_prelude::*;
	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn get_vcs)]
	pub type Attestations<T:Config> = StorageMap<_, Blake2_128Concat, T::Hash, Attestation<T>>;

    #[pallet::config]
    pub trait Config: frame_system::Config + Debug {
		
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

	#[pallet::error]
    pub enum Error<T> {
        /// The claim already exists.
        AlreadyExist,
    }
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		AttestationStored(u32, T::AccountId),

	}

	#[derive(Debug, Encode, Decode, TypeInfo,PartialEq)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct Attestation<T: Config> {
		// hash of the vcs used for this attestation
		vcs_hash: T::Hash,
		// the account execute the attestation
		attester: T::AccountId,
		// revocation status
		revoked: bool,
	}
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn attestation_create(origin: OriginFor<T>, claim_vcs:T::Hash,vcs_hash:T::Hash) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let sender = ensure_signed(origin)?;


			// check if attestation already exist 
			ensure!(!<Attestations<T>>::contains_key(claim_vcs), Error::<T>::AlreadyExist);

			// TODO ==> check if the AccountIdOf can pay for this transaction
			// TODO ==> Validation pre insertion
			
			Attestations::<T>::insert(claim_vcs,Attestation {
				vcs_hash,
				attester :sender.clone(),
				revoked : false

			});
			// Emit an event.

			Self::deposit_event(Event::AttestationStored(1, sender));

			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}


