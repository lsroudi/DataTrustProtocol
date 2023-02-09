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
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn get_vcs)]
	pub type Vcs<T:Config> = StorageMap<_, Blake2_128Concat, T::AccountId, T::Hash>;

    #[pallet::config]
    pub trait Config: frame_system::Config + Debug {
		
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		VcsStored(u32, T::AccountId),

	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn vcs_create(origin: OriginFor<T>, hash:T::Hash) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;



			// TODO ==> check if the AccountIdOf can pay for this transaction
			// TODO ==> Validation pre insertion
			
			Vcs::<T>::insert(&who,hash);
			// Emit an event.

			Self::deposit_event(Event::VcsStored(1, who));

			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}


