#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::ensure_signed;
use codec::{Encode, Decode};
use frame_support::{
	decl_module, decl_storage, decl_event, decl_error, StorageValue, StorageDoubleMap, RuntimeDebug,
	dispatch, traits::Get
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

type Puzzle = u128;
type Amount = u32;

decl_storage! {
	trait Store for Module<T: Trait> as Remittance {
		/// Double map of puzzles with keys of [remitter, puzzle] to [amount]
		pub Deposits get(fn deposits): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) Puzzle => Amount;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		/// Deposit made
		/// [remitter, puzzle, amount]
		Deposit(AccountId, Puzzle, Amount),
		/// Transfer made
		/// [puzzle, remittant, amount]
	    Transfer(Puzzle, AccountId, Amount),
		/// Withdrawal
		/// [remitter, amount]
	   	Withdraw(AccountId, Amount),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		InvalidPassword,
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
		pub fn create_puzzle(origin, password: Puzzle) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			frame_support::ensure!(password > 0, Error::<T>::InvalidPassword);

			// return keccak256(abi.encodePacked(recipient, password1, address(this)));
			Ok(())
		}

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			// Something::put(something);

			// Emit an event.
			//Self::deposit_event(RawEvent::SomethingStored(something, who));
			// Return a successful DispatchResult
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
