#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*, traits::Currency};
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Zero;

    // Storage item for balances of each account
    #[pallet::storage]
    #[pallet::getter(fn balance_of)]
    pub type Balances<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u128, ValueQuery>;

    // Storage item for allowances
    #[pallet::storage]
    #[pallet::getter(fn allowance)]
    pub type Allowances<T: Config> = StorageMap<_, Blake2_128Concat, (T::AccountId, T::AccountId), u128, ValueQuery>;

    // Total supply of the token
    #[pallet::storage]
    #[pallet::getter(fn total_supply)]
    pub type TotalSupply<T: Config> = StorageValue<_, u128, ValueQuery>;

    // Events emitted by this pallet
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        Transfer { from: T::AccountId, to: T::AccountId, value: u128 },
        Approval { owner: T::AccountId, spender: T::AccountId, value: u128 },
    }

    // Errors that can occur in this pallet
    #[pallet::error]
    pub enum Error<T> {
        InsufficientBalance,
        AllowanceExceeded,
        TransferToZeroAddress,
    }

    // Configuration trait for the pallet
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
    }

    // Implementation of the pallet's dispatchable functions
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn transfer(origin: OriginFor<T>, to: T::AccountId, value: u128) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(value > Zero::zero(), "Transfer value must be greater than zero");
            ensure!(to != sender, "Cannot transfer to oneself");
            ensure!(Self::balance_of(&sender) >= value, Error::<T>::InsufficientBalance);

            // Update balances
            Balances::<T>::insert(&sender, Self::balance_of(&sender) - value);
            Balances::<T>::insert(&to, Self::balance_of(&to) + value);

            // Emit transfer event
            Self::deposit_event(Event::Transfer { from: sender.clone(), to, value });
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn approve(origin: OriginFor<T>, spender: T::AccountId, value: u128) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            ensure!(value > Zero::zero(), "Approval value must be greater than zero");
            ensure!(spender != owner, "Cannot approve to oneself");

            // Set allowance
            Allowances::<T>::insert((owner.clone(), spender), value);

            // Emit approval event
            Self::deposit_event(Event::Approval { owner, spender, value });
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn transfer_from(origin: OriginFor<T>, from: T::AccountId, to: T::AccountId, value: u128) -> DispatchResult {
            let spender = ensure_signed(origin)?;

            ensure!(value > Zero::zero(), "Transfer value must be greater than zero");
            ensure!(to != from, "Cannot transfer to oneself");
            ensure!(Self::balance_of(&from) >= value, Error::<T>::InsufficientBalance);
            ensure!(Self::allowance(&from, &spender) >= value, Error::<T>::AllowanceExceeded);

            // Update balances and allowances
            Balances::<T>::insert(&from, Self::balance_of(&from) - value);
            Balances::<T>::insert(&to, Self::balance_of(&to) + value);
            Allowances::<T>::insert((from.clone(), spender), Self::allowance(&from, &spender) - value);

            // Emit transfer event
            Self::deposit_event(Event::Transfer { from, to, value });
            Ok(())
        }
    }
}
