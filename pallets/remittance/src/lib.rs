#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::ensure_signed;
use codec::{Encode, Decode};
use frame_support::{
	decl_module, decl_storage, decl_event, decl_error, StorageValue, StorageDoubleMap, RuntimeDebug,
	dispatch,
	traits::{
		Currency, ExistenceRequirement, Get, ReservableCurrency, WithdrawReason, WithdrawReasons,
	},
};

use sp_runtime::{
	traits::{AccountIdConversion, Saturating, Zero},
	ModuleId,
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

const PALLET_ID: ModuleId = ModuleId(*b"remittan");

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	type Currency: Currency<Self::AccountId>;
}

type AccountIdOf<T> = <T as frame_system::Trait>::AccountId;
type BalanceOf<T> = <<T as Trait>::Currency as Currency<AccountIdOf<T>>>::Balance;
type HashOf<T> = <T as frame_system::Trait>::Hash;

#[derive(Encode, Decode, Default, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Deposit<AccountId, Balance> {
	remitter: AccountId,
	value: Balance,
}

decl_storage! {
	trait Store for Module<T: Trait> as Remittance {
		/// Map of deposits - Hash -> Deposit
		pub Deposits get(fn deposits): 
			map hasher(blake2_128_concat) HashOf<T> => Deposit<AccountIdOf<T>, BalanceOf<T>>;
	}
}

decl_event!(
	pub enum Event<T> where
	Balance = BalanceOf<T>,
	<T as frame_system::Trait>::AccountId,
	<T as frame_system::Trait>::Hash,
	{
		/// Deposit made
		/// [remitter, puzzle, amount]
		Deposit(AccountId, Hash, Balance),
		/// Transfer made
		/// [puzzle, remittant, amount]
	    Transfer(Hash, AccountId, Balance),
		/// Withdrawal
		/// [remitter, amount]
	   	Withdraw(AccountId, Balance),
	   	//Contributed(AccountId, Balance),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		InvalidPuzzle,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn deposit(origin, puzzle: HashOf<T>, value: BalanceOf<T>) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			let account = Self::account_id(puzzle);

			T::Currency::transfer(
				&who,
				&account,
				value,
				ExistenceRequirement::AllowDeath
			)?;

			<Deposits<T>>::insert(puzzle, Deposit {
				remitter: who.clone(),
				value,
			});

			Self::deposit_event(RawEvent::Deposit(who, puzzle, value));

			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn cause_error(origin) -> dispatch::DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			// match Something::get() {
			// 	// Return an error if the value has not been set.
			// 	None => Err(Error::<T>::NoneValue)?,
			// 	Some(old) => {
			// 		// Increment the value read from storage; will error in the event of overflow.
			// 		let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
			// 		// Update the value in storage with the incremented result.
			// 		Something::put(new);
			// 		Ok(())
			// 	},
			// }

			Ok(())
		}
	}
}

impl<T: Trait> Module<T> {
	/// The account ID of the fund pot.
	///
	/// This actually does computation. If you need to keep using it, then make sure you cache the
	/// value and only call this once.
	pub fn account_id(index: HashOf<T>) -> T::AccountId {
		PALLET_ID.into_sub_account(index)
	}
}