#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*, DefaultNoBound};
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{CheckedAdd, One};

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config<I: 'static = ()>: frame_system::Config + pallet_nfts::Config<I> {
        /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_runtime_types/index.html>
        type RuntimeEvent: From<Event<Self, I>>
            + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type RuntimeCall: From<Call<Self, I>> + Encode;

        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: crate::weights::WeightInfo;
    }

    #[pallet::pallet]
    pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

    /// A struct to store a single block-number. Has all the right derives to store it in storage.
    /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_storage_derives/index.html>
    #[derive(
        Encode, Decode, MaxEncodedLen, TypeInfo, CloneNoBound, PartialEqNoBound, DefaultNoBound,
    )]
    #[scale_info(skip_type_params(T, I))]
    pub struct CompositeStruct<T: Config<I>, I: 'static = ()> {
        /// A block number.
        pub(crate) block_number: BlockNumberFor<T>,
        i: PhantomData<I>,
    }

    /// The pallet's storage items.
    /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#storage>
    /// <https://paritytech.github.io/polkadot-sdk/master/frame_support/pallet_macros/attr.storage.html>
    #[pallet::storage]
    pub type Something<T: Config<I>, I: 'static = ()> = StorageValue<_, CompositeStruct<T, I>>;

    /// Pallets use events to inform users when important changes are made.
    /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#event-and-error>
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config<I>, I: 'static = ()> {
        /// We usually use passive tense for events.
        SomethingStored {
            block_number: BlockNumberFor<T>,
            who: T::AccountId,
        },
    }

    /// Errors inform users that something went wrong.
    /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#event-and-error>
    #[pallet::error]
    pub enum Error<T, I = ()> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
    }

    /// Dispatchable functions allows users to interact with the pallet and invoke state changes.
    /// These functions materialize as "extrinsics", which are often compared to transactions.
    /// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#dispatchables>

    #[pallet::call]
    impl<T: Config<I>, I: 'static> Pallet<T, I> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn do_something(origin: OriginFor<T>, bn: u32) -> DispatchResultWithPostInfo {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_origin/index.html>
            let who = ensure_signed(origin)?;

            // Convert the u32 into a block number. This is possible because the set of trait bounds
            // defined in [`frame_system::Config::BlockNumber`].
            let block_number: BlockNumberFor<T> = bn.into();

            // Update storage.
            <Something<T, I>>::put(CompositeStruct {
                block_number,
                i: PhantomData,
            });

            // Emit an event.
            Self::deposit_event(Event::SomethingStored { block_number, who });

            // Return a successful [`DispatchResultWithPostInfo`] or [`DispatchResult`].
            Ok(().into())
        }

        /// An example dispatchable that may throw a custom error.
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().reads_writes(1,1))]
        pub fn cause_error(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let _who = ensure_signed(origin)?;

            // Read a value from storage.
            match <Something<T, I>>::get() {
                // Return an error if the value has not been set.
                None => Err(Error::<T, I>::NoneValue)?,
                Some(mut old) => {
                    // Increment the value read from storage; will error in the event of overflow.
                    old.block_number = old
                        .block_number
                        .checked_add(&One::one())
                        // ^^ equivalent is to:
                        // .checked_add(&1u32.into())
                        // both of which build a `One` instance for the type `BlockNumber`.
                        .ok_or(Error::<T, I>::StorageOverflow)?;
                    // Update the value in storage with the incremented result.
                    <Something<T, I>>::put(old);
                    // Explore how you can rewrite this using
                    // [`frame_support::storage::StorageValue::mutate`].
                    Ok(().into())
                }
            }
        }
    }
}
