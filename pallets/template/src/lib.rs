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
	//use frame_benchmarking::log::kv::Value;
use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::{vec::Vec, fmt::*};
	//use uuid::Uuid;
	

	#[derive(Debug, Clone, PartialEq, Default, Encode, Decode)]
	pub struct Book<AccountId> {
		author: AccountId,
		title: Vec<u8>,
		cover_image_hash: Vec<u8>,
		total_chapters: u32,
		total_pages: u32,
	}

	#[derive(Debug, Clone, PartialEq, Default, Encode, Decode)]
	pub struct Chapter<AccountId> {
		book_ref: AccountId,
		chapter_id: u32,
		description: Vec<u8>,
		pages: Vec<u32>,
	}

	#[derive(Debug, Clone, PartialEq, Default, Encode, Decode)]
	pub struct Page<AccountId> {
		book_ref: AccountId,
		chapter: u32,
		page_num: u32,
		content: Vec<u8>,
	}

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
	#[pallet::getter(fn get_book)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub (super) type Books<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Book<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_chapter)]
	pub (super) type Chapters<T: Config> = StorageDoubleMap<_, Twox64Concat, T::AccountId, Twox64Concat, u32, Chapter<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	pub type NextChapter<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, u32>;

	#[pallet::storage]
	#[pallet::getter(fn get_page)]
	pub (super) type Pages<T: Config> = StorageDoubleMap<_, Twox64Concat, T::AccountId, Twox64Concat, u32, Page<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	pub type NextPage<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		BookCreated(T::AccountId),
		ChapterCreated(u32, T::AccountId),
		PageCreated(u32, u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		InvalidChapter,
	}

	#[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn create_book(origin: OriginFor<T>, title: Vec<u8>, cover_image_hash: Vec<u8>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;
			let who1 = who.clone();
			let who2 = who.clone();
			let page_count = NextPage::<T>::get(who.clone()).unwrap_or(0);
			let chapter_count = NextChapter::<T>::get(who.clone()).unwrap_or(0);
			//let _uuid = Uuid::new_v5(&Uuid::NAMESPACE_OID, &title);
			// Update storage.
			Books::<T>::insert(who2, Book {
				author: who,
				title,
				cover_image_hash,
				total_chapters: chapter_count,
				total_pages: page_count,
			});

			// Emit an event.
			Self::deposit_event(Event::BookCreated(who1));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,2))]
		pub fn create_chapter(origin: OriginFor<T>, book_ref: T::AccountId, description: Vec<u8>) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			let chapter = NextChapter::<T>::get(_who.clone()).unwrap_or(0);
			let _ref = book_ref.clone();
			let _ref1 = book_ref.clone();
			Chapters::<T>::insert(_ref, chapter, Chapter {
				book_ref: _who,
				chapter_id: chapter,
				description,
				pages: Vec::new(),
			});
			NextChapter::<T>::insert(book_ref.clone(), chapter + 1);
			let mut book = Books::<T>::get(book_ref.clone());
			book.total_pages += 1;
			let pages = book.total_pages.clone();
			Books::<T>::insert(book_ref.clone(), book);
			Self::deposit_event(Event::ChapterCreated(pages, book_ref.clone()));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,3))]
		pub fn create_page(origin: OriginFor<T>, book_ref: T::AccountId, chapter: u32, content: Vec<u8>) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			let _who1 = _who.clone();

			let _ref = &book_ref.clone();
			let page = NextPage::<T>::get(_who.clone()).unwrap_or(0);
			let mut _chapter = Chapters::<T>::get(_ref, chapter);
			ensure!(_chapter.chapter_id.ge(&0), Error::<T>::InvalidChapter);
			Pages::<T>::insert(_ref, page, Page {
				book_ref: _who,
				chapter,
				page_num: page,
				content,
			});
			_chapter.pages.push(page);
			Chapters::<T>::insert(_ref, chapter, _chapter);
			NextPage::<T>::insert(_who1, page + 1);
			let mut book = Books::<T>::get(book_ref.clone());
			book.total_pages +=1;
			let pages = book.total_pages.clone();
			Books::<T>::insert(book_ref.clone(), book);
			Self::deposit_event(Event::PageCreated(pages, chapter.clone(), book_ref.clone()));
			Ok(())
		}
	}
}
