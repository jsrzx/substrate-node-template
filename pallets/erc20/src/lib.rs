#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Codec, Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure, Parameter,
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::traits::{AtLeast32BitUnsigned, CheckedAdd, CheckedSub, Member};
use sp_std::prelude::*;

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type TokenBalance: CheckedAdd
        + CheckedSub
        + Parameter
        + Member
        + Codec
        + Default
        + Copy
        + AtLeast32BitUnsigned;
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Debug)]
pub struct Erc20Token<U> {
    name: Vec<u8>,
    ticker: Vec<u8>,
    total_supply: U,
}

decl_storage! {
    trait Store for Module<T: Trait> as Erc20 {
        Tokens get(fn token_details): Erc20Token<T::TokenBalance>;
        BalanceOf get(fn balance_of): map hasher(blake2_128_concat) T::AccountId => T::TokenBalance;
        Allowance get(fn allowance): map hasher(blake2_128_concat) (T::AccountId, T::AccountId) => T::TokenBalance;
    }
}

decl_event!(
    //FIXME: pub enum Event<T> where AccountId = <T as system::Trait>::AccountId, <T as self::Trait>::TokenBalance {
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
        Balance = <T as self::Trait>::TokenBalance,
    {
        // from, to, value
        Transfer(AccountId, AccountId, Balance),
        // owner, spender, value
        Approval(AccountId, AccountId, Balance),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        StorageOverflow,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        // 资产发行
        #[weight = 0]
        fn init(origin, name: Vec<u8>, ticker: Vec<u8>, total_supply: T::TokenBalance) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(name.len() <= 64, "token name cannot exceed 64 bytes");
            ensure!(ticker.len() <= 32, "token ticker cannot exceed 32 bytes");

            let token = Erc20Token {
                name,
                ticker,
                total_supply,
            };

            <Tokens<T>>::set(token);
            <BalanceOf<T>>::insert(sender, total_supply);

            Ok(())
        }

        // 转账接口
        #[weight = 0]
        fn transfer(_origin, to: T::AccountId, value: T::TokenBalance) -> DispatchResult {
            let sender = ensure_signed(_origin)?;
            Self::_transfer(sender, to, value)
        }

        // 替人转账接口
        #[weight = 0]
        pub fn transfer_from(_origin, from: T::AccountId, to: T::AccountId, value: T::TokenBalance) -> DispatchResult {
            //ensure!(<Allowance<T>>::exists((from.clone(), to.clone())), "Allowance does not exist.");
            let allowance = Self::allowance((from.clone(), to.clone()));
            ensure!(allowance >= value, "Not enough allowance.");

            // using checked_sub (safe math) to avoid overflow
            //let updated_allowance = allowance.checked_sub(&value).ok_or("overflow in calculating allowance")?;
            let updated_allowance = allowance.checked_sub(&value).ok_or(Error::<T>::StorageOverflow)?;
            <Allowance<T>>::insert((from.clone(), to.clone()), updated_allowance);

            Self::deposit_event(RawEvent::Approval(from.clone(), to.clone(), value));
            Self::_transfer(from, to, value)
        }

        // 授权接口
        #[weight = 0]
        fn approve(_origin, spender: T::AccountId, value: T::TokenBalance) -> DispatchResult {
            let sender = ensure_signed(_origin)?;
            //ensure!(<BalanceOf<T>>::exists(sender.clone()), "Account does not own this token");

            let allowance = Self::allowance((sender.clone(), spender.clone()));
            let updated_allowance = allowance.checked_add(&value).ok_or(Error::<T>::StorageOverflow)?;
            <Allowance<T>>::insert((sender.clone(), spender.clone()), updated_allowance);

            Self::deposit_event(RawEvent::Approval(sender.clone(), spender.clone(), value));

            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    // the ERC20 standard transfer function
    // internal
    fn _transfer(from: T::AccountId, to: T::AccountId, value: T::TokenBalance) -> DispatchResult {
        //ensure!(
        //    <BalanceOf<T>>::exists(from.clone()),
        //    "Account does not own this token"
        //);
        let sender_balance = Self::balance_of(from.clone());
        ensure!(sender_balance >= value, "Not enough balance.");

        let updated_from_balance = sender_balance
            .checked_sub(&value)
            .ok_or(Error::<T>::StorageOverflow)?;
        let receiver_balance = Self::balance_of(to.clone());
        let updated_to_balance = receiver_balance
            .checked_add(&value)
            .ok_or(Error::<T>::StorageOverflow)?;

        // reduce sender's balance
        <BalanceOf<T>>::insert(from.clone(), updated_from_balance);

        // increase receiver's balance
        <BalanceOf<T>>::insert(to.clone(), updated_to_balance);

        Self::deposit_event(RawEvent::Transfer(from, to, value));
        Ok(())
    }
}
