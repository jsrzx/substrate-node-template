#![cfg_attr(not(feature = "std"), no_std)]

// 1. Imports
use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, traits::Get, StorageMap,
    StorageValue,
};
use frame_system::ensure_signed;
use sp_io::hashing::blake2_128;
use sp_runtime::DispatchError;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

type KittyIndex = u32;

// kitty的DNA数据
#[derive(Encode, Decode)]
pub struct Kitty(pub [u8; 16]);

// 2. Pallet Configuration
pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type Randomness: Randomness<Self::Hash>;
}

// 3. Pallet Storage Items
decl_storage! {
    pub Kitties get(fn kitties): map hasher(blake2_128_concat) KittyIndex => Option<Kitty>;
    pub KittiesCount get(fn kitties_count): KittyIndex;
    pub KittyOwner get(fn kitty_owner): map hasher(blake2_128_concat) KittyIndex => Option<T::AccountId>
}

// 4. Pallet Events
decl_event! {
    pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
    }
}

// 5. Pallet Errors
decl_error! {
    pub enum Error for Module<T: Trait> {
    }
}

// 6. Callable Pallet Functions
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing errors
        // this includes information about your errors in the node's metadata.
        // it is needed only if you are using errors in your pallet
        type Error = Error<T>;

        // A default function for depositing events
        fn deposit_event() = default;

        // 创建小猫
        #[weight=0]
        pub fn create(origin) {
            let sender = ensure_signed(origin)?;
            let kitty_id = Self::next_kitty_id()?;
        }

    }
}

impl<T: Trait> Module<T> {
    fn next_kitty_id() -> sp_std::result::Result<KittyIndex, DispatchError> {
        let kitty_id = Self::kitties_count();
        if kitty_id == KittyIndex::max_value() {}
    }
}
