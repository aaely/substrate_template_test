#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub type Books<T: Config> = StorageMap<_, Twox64Concat, u32, Book<T::AccountId>>;
	
	#[pallet::storage]
	pub type NextBook<T> = StorageValue<_, u32>;

	#[pallet::storage]
	pub type Chapters<T: Config> = StorageMap<_, Twox64Concat, u32, Chapter>;

	#[pallet::storage]
	pub type NextChapter<T> = StorageValue<_, u32>;

	#[pallet::storage]
	pub type Pages<T: Config> = StorageMap<_, Twox64Concat, u32, Page>;

	#[pallet::storage]
	pub type NextPage<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		NewBook(u32, Book::<T::AccountId>),
		NewChapter(u32, Chapter),
		NewPage(u32, Page),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(0)]
		pub fn new_book(origin: OriginFor<T>, title: Vec<u8>, cover_image_hash: Vec<u8>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			let book: u32 = NextBook::<T>::get().unwrap_or(0);
			Books::<T>::insert(book, Book::<T::AccountId> {
				author: who,
				book_ref: book,
				title,
				cover_image_hash,
			});
			NextBook::<T>::put(book + 1);
			// Emit an event.
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn new_chapter(origin: OriginFor<T>, description: Vec<u8>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			let chapter: u32 = NextChapter::<T>::get().unwrap_or(0);
			let new_slice: Vec<u32> = Vec::new();
			let book_id: u32 = NextBook::<T>::get().unwrap_or(0);
			Chapters::<T>::insert(chapter, Chapter {
				chapter_id: chapter,
				book_ref: book_id,
				description,
				pages: new_slice,
			});
			NextChapter::<T>::put(chapter + 1);
			// Emit an event.
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn new_page(origin: OriginFor<T>, content: Vec<u8>, book: u32, chapter: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			let page: u32 = NextPage::<T>::get().unwrap_or(0);
			Pages::<T>::insert(page, Page {
				book_ref: book,
				page_number: page,
				chapter_id: chapter,
				content,
			});
			NextPage::<T>::put(page + 1);
			// Emit an event.
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}

	use v1::{Book, Chapter, Page};
	pub mod v1 {
		use codec::{Encode, Decode};
		use sp_std::vec::Vec;

		#[derive(Encode, Decode, Debug, Clone, PartialEq)]
		pub struct Book<AccountId> {
			pub title: Vec<u8>,
			pub book_ref: u32,
			pub cover_image_hash: Vec<u8>,
			pub author: AccountId,
		}

		#[derive(Encode, Decode, Debug, Clone, PartialEq)]
		pub struct Chapter {
			pub book_ref: u32,
			pub chapter_id: u32,
			pub description: Vec<u8>,
			pub pages: Vec<u32>,
		}

		#[derive(Encode, Decode, Debug, Clone, PartialEq)]
		pub struct Page {
			pub book_ref: u32,
			pub page_number: u32,
			pub chapter_id: u32,
			pub content: Vec<u8>,
		}
	}
}
