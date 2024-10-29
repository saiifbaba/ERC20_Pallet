
use frame_support::{
    decl_module, decl_storage, decl_event, decl_error, dispatch, ensure,
};
use frame_system::ensure_signed;
pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}
decl_storage! {
    trait Store for Module<T: Config> as ERC20 {
        // Total supply of tokens.
        TotalSupply get(fn total_supply): u64;
        // Balances for each account.
        Balances get(fn balance_of): map hasher(blake2_128_concat) T::AccountId => u64;
    }
}
decl_event!(
    pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
        Transfer(AccountId, AccountId, u64),
    }
);
decl_error! {
    pub enum Error for Module<T: Config> {
        InsufficientBalance,
    }
}
decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        #[weight = 10_000]
        pub fn transfer(origin, to: T::AccountId, amount: u64) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let sender_balance = Self::balance_of(&sender);
            ensure!(sender_balance >= amount, Error::<T>::InsufficientBalance);
            
            <Balances<T>>::insert(&sender, sender_balance - amount);
            let receiver_balance = Self::balance_of(&to);
            <Balances<T>>::insert(&to, receiver_balance + amount);

            Self::deposit_event(RawEvent::Transfer(sender, to, amount));
            Ok(())
        }
    }
}


