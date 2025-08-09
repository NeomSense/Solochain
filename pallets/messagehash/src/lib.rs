#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*, BoundedVec};
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Max items (or bytes) allowed. Use consistently with storage bounds.
        #[pallet::constant]
        type MaxItems: Get<u32>;

        /// Max byte length allowed for the stored vector.
        #[pallet::constant]
        type MaxLen: Get<u32>;
    }

    /// A single bounded blob of bytes.
    #[pallet::storage]
    #[pallet::getter(fn stored_value)]
    pub type StoredValue<T: Config> = StorageValue<_, BoundedVec<u8, T::MaxLen>, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Data stored with its length.
        Stored(u32),
        /// Data cleared.
        Cleared,
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Input exceeded T::MaxLen
        TooLong,
        /// Numeric cap exceeded (if you keep a separate numeric guard)
        TooManyItems,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Store a bounded byte array.
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)] // TODO: replace with benchmarked weights
        pub fn store(origin: OriginFor<T>, data: Vec<u8>) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // Example numeric guard (optional, if you need it)
            ensure!( (data.len() as u32) <= T::MaxItems::get(), Error::<T>::TooManyItems );

            let bounded: BoundedVec<_, T::MaxLen> =
                data.try_into().map_err(|_| Error::<T>::TooLong)?;

            <StoredValue<T>>::put(bounded);
            let len = StoredValue::<T>::get().map(|v| v.len() as u32).unwrap_or(0);
            Self::deposit_event(Event::Stored(len));
            Ok(())
        }

        /// Clear the stored value.
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)] // TODO: replace with benchmarked weights
        pub fn clear(origin: OriginFor<T>) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            <StoredValue<T>>::kill();
            Self::deposit_event(Event::Cleared);
            Ok(())
        }
    }
}
