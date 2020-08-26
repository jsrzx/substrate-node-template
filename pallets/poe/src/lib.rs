#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, StorageMap,
};
use frame_system::{self as system, ensure_signed};
use sp_std::vec::Vec;

// 2. Pallet Configuration
pub trait Trait: system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// 4. Pallet Events
decl_event! {
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
        /// Event emitted when a proof has been claimed.
        ClaimCreated(AccountId, Vec<u8>),
        /// Event emitted when a claim is revoked by the owner.
        ClaimRevoked(AccountId, Vec<u8>),
        /// Event emitted when a claim is transfer by the owner.
        ClaimTransfer(AccountId, Vec<u8>),
    }
}

// 5. Pallet Errors
decl_error! {
    pub enum Error for Module<T: Trait> {
        /// This proof has already been claimed
        ProofAlreadyClaimed,
        /// The proof does not exist, so it cannot be revoked
        NoSuchProof,
        /// The proof is claimed by another account, so caller can't revoke it
        NotProofOwner,
        /// not claim owner
        NotClaimOwner,
        ClaimNotExist,

    }
}

// 3. Pallet Storage Items
decl_storage! {
    trait Store for Module<T: Trait> as TemplateModule {
        /// The storage item for our proofs.
        /// It maps a proof to the user who made the claim and when they made it.
        Proofs: map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
    }
}

// 6. Callable Pallet Functions
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

    // 创建存证
        #[weight = 10_000]
        fn create_claim(origin, proof: Vec<u8>) {
            let sender = ensure_signed(origin)?;

            ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

            let current_block = <system::Module<T>>::block_number();

            Proofs::<T>::insert(&proof, (&sender, current_block));

            Self::deposit_event(RawEvent::ClaimCreated(sender, proof));
        }

    // 撤销存证
        #[weight = 10_000]
        fn revoke_claim(origin, proof: Vec<u8>) {
            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

            let (owner, _) = Proofs::<T>::get(&proof);

            ensure!(sender == owner, Error::<T>::NotProofOwner);

            Proofs::<T>::remove(&proof);

            Self::deposit_event(RawEvent::ClaimRevoked(sender, proof));
        }

    // 转移存证
        #[weight = 10_000]
        pub fn transfer_claim(origin, claim: Vec<u8>, receiver: T::AccountId) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            // 存证不存在
            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

            // 不是拥有者
            let (owner, block_number) = Proofs::<T>::get(&claim);
            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            Proofs::<T>::remove(&claim);
            Proofs::<T>::insert(&claim, (receiver.clone(), block_number));

            Self::deposit_event(RawEvent::ClaimTransfer(receiver, claim));

            Ok(())
        }
    }
}
