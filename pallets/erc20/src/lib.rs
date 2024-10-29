
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

