#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    pallet_prelude::*,
    traits::Get,
    BoundedVec,
    weights::{Weight, constants::RocksDbWeight},
};
use frame_system::pallet_prelude::*;
use sp_std::{marker::PhantomData, vec::Vec};

/// --- Inline weights module so you don't need a separate weights.rs file ---
mod weights {
    use super::*;

    /// Weight trait for pallet-por. Add one fn per extrinsic/hook.
    pub trait WeightInfo {
        fn store() -> Weight;
        fn clear() -> Weight;
    }

    /// Generic weight impl (recommended in runtime):
    ///   type WeightInfo = pallet_por::SubstrateWeight<Self>;
    pub struct SubstrateWeight<T>(pub PhantomData<T>);
    impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
        fn store() -> Weight {
            // Placeholder exec cost + 1 DB write. Replace via FRAME benchmarks.
            Weight::from_parts(10_000, 0)
                .saturating_add(T::DbWeight::get().writes(1))
        }
        fn clear() -> Weight {
            Weight::from_parts(10_000, 0)
                .saturating_add(T::DbWeight::get().writes(1))
        }
    }

    /// Fallback weights (used if runtime sets `type WeightInfo = ();`).
    impl WeightInfo for () {
        fn store() -> Weight {
            Weight::from_parts(10_000, 0)
                .saturating_add(RocksDbWeight::get().writes(1))
        }
        fn clear() -> Weight {
            Weight::from_parts(10_000, 0)
                .saturating_add(RocksDbWeight::get().writes(1))
        }
    }
}
pub use weights::{SubstrateWeight, WeightInfo};

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    /// Pallet configuration.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Standard event wire-up.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Max length for stored bytes.
        #[pallet::constant]
        type MaxItemLen: Get<u32>;

        /// Weight provider (benchmarked or fallback).
        type WeightInfo: WeightInfo;
    }

    /// Pallet type.
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Bounded byte vector used by storage/extrinsics.
    pub type ItemOf<T> = BoundedVec<u8, <T as Config>::MaxItemLen>;

    /// Single storage item.
    #[pallet::storage]
    #[pallet::getter(fn item)]
    pub type Item<T: Config> = StorageValue<_, ItemOf<T>, ValueQuery>;

    /// Events.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        Stored { who: T::AccountId, len: u32 },
        Cleared { who: T::AccountId },
    }

    /// Errors.
    #[pallet::error]
    pub enum Error<T> {
        TooLong,
        NothingToClear,
    }

    /// Calls.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Store a bounded byte vector.
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::store())]
        pub fn store(origin: OriginFor<T>, data: Vec<u8>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let bounded: ItemOf<T> = data.try_into().map_err(|_| Error::<T>::TooLong)?;
            Item::<T>::put(bounded);
            let len = Item::<T>::get().len() as u32;
            Self::deposit_event(Event::Stored { who, len });
            Ok(())
        }

        /// Clear the stored value.
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::clear())]
        pub fn clear(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Item::<T>::exists(), Error::<T>::NothingToClear);
            Item::<T>::kill();
            Self::deposit_event(Event::Cleared { who });
            Ok(())
        }
    }
}
