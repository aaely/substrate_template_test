#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;
/*#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mosp_runtime::{offchain::storage::StorageValueRef, traits::StaticLookup}b mod pallet {
*/
//use frame_benchmarking::log::kv::Value;
#[frame_support::pallet]
pub mod pallet {
use frame_support::{Twox64Concat, pallet_prelude::*};
use frame_system::{ensure_signed, pallet_prelude::*};
use codec::{Encode, Decode};
	
	use sp_runtime::offchain::storage::StorageValueRef;
use sp_std::prelude::*;
	
	#[derive(Debug, Clone, PartialEq, Default, Encode, Decode)]
	pub struct User<AccountId> {
		address: AccountId,
		fname: Vec<u8>,
		lname: Vec<u8>,
		phone: Vec<u8>,
		email: Vec<u8>,
		handle: Vec<u8>,
		handle_id: u128,
		bio: Vec<u8>,
		website: Vec<u8>,
		profile_image: Vec<u8>,
		total_orders: u32,
	}

	#[derive(Debug, Clone, PartialEq, Default, Encode, Decode)]
	pub struct SellerStatistics<AccountId> {
		reviewer: AccountId,
		avg_rating: u32,
		product_ref: u128,
		total_stars: u32,
		total_reviews: u32,
	}

	#[derive(Debug, Clone, PartialEq, Default, Encode, Decode)]
	pub struct ProductStatistics<AccountId> {
		reviewer: AccountId,
		avg_rating: u32,
		total_stars: u32,
		total_reviews: u32,
	}

	#[derive(Debug, Clone, PartialEq, Default, Encode, Decode)]
	pub struct Review<AccountId> {
		reviewer: AccountId,
		rating: u32,
		product_ref: u128,
		review: Vec<u8>,
	}

	#[derive(Debug, Clone, PartialEq, Encode, Decode, Default)]
	pub struct Post<AccountId> {
		author: AccountId,
		likes: u32,
		handle_tags: Vec<u128>,
		hashtags: Vec<u128>,
		content: Vec<u8>,
		comments: Vec<u128>,
		total_comments: u32,
		images: Vec<Vec<u8>>,
	}

	/*impl Order {
		pub fn build(
			id: &u128, 
			user: &T::AccountId, 
			cannabis_products:&Vec<(u128, u32)>, 
			peptide_products: &Vec<(u128, u32)>, 
			total: u32,
			date: Vec<u8>) -> Order {
			
		}
	}*/

	#[derive(Debug, Clone, PartialEq, Default, Encode, Decode)]
	pub struct Order<AccountId> {
		id: u128,
		user: AccountId,
		products: Vec<(Vec<u8>, u32, u32)>,
		total: u32,
		date: Vec<u8>,
	}

	#[derive(Debug, Clone, PartialEq, Default, Encode, Decode)]
	pub struct Peptide<AccountId> {
		created_by: AccountId,
		id: u128,
		name: Vec<u8>,
		price: u32,
		inventory: u32,
		image_hash: Vec<u8>,
	}

	#[derive(Debug, Clone, PartialEq, Default, Encode, Decode)]
	pub struct PeptideProfile<AminoAcid> {
		peptide_ref: u128,
		chain: Vec<AminoAcid>,
		production_cost: u32,
		production_yield: u32,
	}

	#[derive(Debug, Clone, PartialEq, Default, Encode, Decode)]
	pub struct AminoAcid {
		id: u128,
		name: Vec<u8>,
		cost: u32,
	}

	#[derive(Debug, Clone, PartialEq, Encode, Decode)]
	pub struct CannabisProduct {
		id: u128,
		name: Vec<u8>,
		price: u32,
		category: CannabisCategory,
		inventory: u32,
		image_hash: Vec<u8>,
		cannabinoids: Vec<(u128, Vec<u8>, u32)>,
		terpenes: Vec<(u128, Vec<u8>, u32)>,
	}

	impl Default for CannabisProduct {
		fn default() -> Self {
			CannabisProduct {
				id: Default::default(),
				name: Default::default(),
				price: Default::default(),
				category: CannabisCategory::Flower,
				inventory: Default::default(),
				image_hash: Default::default(),
				cannabinoids: Default::default(),
				terpenes: Default::default(),
			}

		}
	}

	#[derive(Debug, Clone, PartialEq, Default, Encode, Decode)]
	pub struct Cannabinoid {
		id: u128,
		name: Vec<u8>,
		description: Vec<u8>,
		products: Vec<(u128, u32)>,
	}

	#[derive(Debug, Clone, PartialEq, Default, Encode, Decode)]
	pub struct Terpene {
		id: u128,
		name: Vec<u8>,
		description: Vec<u8>,
		products: Vec<(u128, u32)>,
	}

	#[derive(Debug, Clone, PartialEq, Encode, Decode)]
	pub enum CannabisCategory {
		Flower,
		CO2Extract,
		ButaneExtract,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_balances::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn get_peptide)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub (super) type Peptides<T: Config> = StorageMap<_, Twox64Concat, u128, (Peptide<T::AccountId>, PeptideProfile<AminoAcid>), ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_amino)]
	pub (super) type AminoAcids<T> = StorageMap<_, Twox64Concat, u128, AminoAcid, ValueQuery>;

	#[pallet::storage]
	pub type PeptideByCount<T: Config> = StorageMap<_, Twox64Concat, u32, (Peptide<T::AccountId>, PeptideProfile<AminoAcid>)>;

	#[pallet::storage]
	pub (super) type AminoAcidByCount<T> = StorageMap<_, Twox64Concat, u32, AminoAcid>;

	#[pallet::storage]
	pub type PeptideCount<T: Config> = StorageValue<_, u32>;

	#[pallet::storage]
	pub (super) type AminoAcidCount<T> = StorageValue<_, u32>;
	
	#[pallet::storage]
	#[pallet::getter(fn get_cannabis_product)]
	pub (super) type CannabisProducts<T> = StorageMap<_, Twox64Concat, u128, CannabisProduct>;

	#[pallet::storage]
	pub type CannabisCount<T: Config> = StorageValue<_, u32>;

	#[pallet::storage]
	pub type CannabisProductByCount<T> = StorageMap<_, Twox64Concat, u32, CannabisProduct>;

	#[pallet::storage]
	#[pallet::getter(fn get_terpene)]
	pub (super) type Terpenes<T> = StorageMap<_, Twox64Concat, u128, Terpene>;

	#[pallet::storage]
	#[pallet::getter(fn get_cannabinoid)]
	pub (super) type Cannabinoids<T> = StorageMap<_, Twox64Concat, u128, Cannabinoid>;

	#[pallet::storage]
	pub type TerpeneByCount<T> = StorageMap<_, Twox64Concat, u32, Terpene>;

	#[pallet::storage]
	pub type CannabinoidByCount<T> = StorageMap<_, Twox64Concat, u32, Cannabinoid>;

	#[pallet::storage]
	pub type TerpeneCount<T> = StorageValue<_, u32>;

	#[pallet::storage]
	pub type UserCount<T> = StorageValue<_, u32>;

	#[pallet::storage]
	pub type CannabinoidCount<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn get_admin)]
	pub (super) type Admins<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, bool, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_user_access)]
	pub (super) type UserAccess<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, bool, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_user)]
	pub (super) type Users<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, User<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	pub type Owner<T: Config> = StorageValue<_, T::AccountId>;

	#[pallet::storage]
	pub type Orders<T: Config> = StorageMap<_, Twox64Concat, u128, Order<T::AccountId>>;

	#[pallet::storage]
	pub type OrdersByUser<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Vec<Order<T::AccountId>>>;

	#[pallet::storage]
	pub type ProductReviews<T: Config> = StorageMap<_, Twox64Concat, u128, ProductStatistics<T::AccountId>>;

	#[pallet::storage]
	pub type ProductReviewCount<T> = StorageValue<_, u128>;
	
	#[pallet::storage]
	pub (super) type OrderCount<T> = StorageValue<_, u128>;

	#[pallet::storage]
	pub type UserHandleAvailability<T> = StorageMap<_, Twox64Concat, u128, bool>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		NewPeptide(u32, T::AccountId),
		NewAmino(u32, T::AccountId),
		PeptideInventoryUpdate((Peptide<T::AccountId>, PeptideProfile<AminoAcid>)),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		InvalidChapter,
		InsufficientAmount,
		ItemAlreadyExists,
		UserAlreadyExists,
		InsufficientPriv,
		HandleAlreadyExists,
	}

	#[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,2))]
		pub fn create_peptide(
			origin: OriginFor<T>, 
			name: Vec<u8>, 
			id: u128, 
			price: u32, 
			inventory: u32, 
			image_hash: Vec<u8>,
			chain: Vec<AminoAcid>) -> DispatchResult {
				// Check that the extrinsic was signed and get the signer.
				// This function will return an error if the extrinsic is not signed.
				// https://substrate.dev/docs/en/knowledgebase/runtime/origin
				let who = ensure_signed(origin)?;
				let who1 = &who.clone();
				let who2 = who.clone();
				ensure!(!Self::check_duplicate_peptide(&id), Error::<T>::ItemAlreadyExists);
				let count = PeptideCount::<T>::get().unwrap_or(0);
				let production_cost = Self::production_cost_calc(&chain).0;
				let production_yield = Self::production_cost_calc(&chain).1;
				let name1 = name.clone();
				let chain1 = chain.clone();
				let image_hash1 = image_hash.clone();
				// Update storage.
				Peptides::<T>::insert(id.clone(), (Peptide {
					created_by: who,
					id: id.clone(),
					name,
					price,
					inventory,
					image_hash
				}, PeptideProfile {
					peptide_ref: id.clone(),
					chain,
					production_cost,
					production_yield,
				}));

				PeptideByCount::<T>::insert(count.clone(), (Peptide {
					created_by: who1.clone(),
					id: id.clone(),
					name: name1,
					price,
					inventory,
					image_hash: image_hash1,
				}, PeptideProfile {
					peptide_ref: id,
					chain: chain1,
					production_cost,
					production_yield,
				}));

				PeptideCount::<T>::put(count + 1);

				// Emit an event
				Self::deposit_event(Event::NewPeptide(count.clone(), who2));
				// Return a successful DispatchResultWithPostInfo
				Ok(())
			}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,3))]
		pub fn create_amino(
			origin: OriginFor<T>, 
			name: Vec<u8>, 
			id: u128, 
			cost: u32) -> DispatchResult {
				let who = ensure_signed(origin)?;
				ensure!(!Self::check_duplicate_amino(&id), Error::<T>::ItemAlreadyExists);
				let count = AminoAcidCount::<T>::get().unwrap_or(0);
				let count1 = count.clone();
				let id1 = id.clone();
				let cost1 = cost.clone();
				let name1 = name.clone();
				AminoAcids::<T>::insert(id, AminoAcid {
					id,
					name,
					cost,
				});
				AminoAcidByCount::<T>::insert(count, AminoAcid {
					id: id1,
					name: name1,
					cost: cost1,
				});
				Self::deposit_event(Event::NewAmino(count1.clone(), who));
				AminoAcidCount::<T>::put(count1.clone() + 1);
				Ok(())
			}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,2))]
		pub fn new_cannabis_product(
			origin: OriginFor<T>, 
			id: u128, 
			name: Vec<u8>, 
			price: u32, 
			category: CannabisCategory, 
			inventory: u32, 
			image_hash: Vec<u8>, 
			cannabinoids: Vec<(u128, Vec<u8>, u32)>, 
			terpenes: Vec<(u128, Vec<u8>, u32)>) -> DispatchResult {
				ensure_signed(origin)?;
				ensure!(!Self::check_duplicate_cannabis(&id), Error::<T>::ItemAlreadyExists);
				let count = CannabisCount::<T>::get().unwrap_or(0);
				Self::add_product_to_cannabinoid(&id, &cannabinoids);
				Self::add_product_to_terpene(&id, &terpenes);
				CannabisProducts::<T>::insert(id.clone(), CannabisProduct {
					id: id.clone(),
					name: name.clone(),
					price: price.clone(),
					category: category.clone(),
					inventory: inventory.clone(),
					image_hash: image_hash.clone(),
					cannabinoids: cannabinoids.clone(),
					terpenes: terpenes.clone(),
				});
				CannabisProductByCount::<T>::insert(count.clone(), CannabisProduct {
					id: id.clone(),
					name: name.clone(),
					price: price.clone(),
					category: category.clone(),
					inventory: inventory.clone(),
					image_hash: image_hash.clone(),
					cannabinoids: cannabinoids.clone(),
					terpenes: terpenes.clone(),
				});
				CannabisCount::<T>::put(count + 1);
				Ok(())
		}
		
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,2))]
		pub fn new_terpene(
			origin: OriginFor<T>, 
			id: u128, 
			name: Vec<u8>, 
			description: Vec<u8>) -> DispatchResult {
				let who = ensure_signed(origin)?;
				//ensure!(Self::get_admin(who.clone()), Error::<T>::InsufficientPriv);
				ensure!(!Self::check_duplicate_terpene(&id), Error::<T>::ItemAlreadyExists);
				let count = TerpeneCount::<T>::get().unwrap_or(0);
				let id1 = id.clone();
				let name1 = name.clone();
				let description1 = description.clone();
				Terpenes::<T>::insert(id, Terpene {
					id,
					name,
					description,
					products: Vec::new(),
				});
				TerpeneByCount::<T>::insert(count.clone(), Terpene {
					id: id1,
					name: name1,
					description: description1,
					products: Vec::new(),
				});
				TerpeneCount::<T>::put(count + 1);
				Ok(())
		}
		
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,2))]
		pub fn new_cannabinoid(
			origin: OriginFor<T>, 
			id: u128, 
			name: Vec<u8>, 
			description: Vec<u8>) -> DispatchResult {
				let who = ensure_signed(origin)?;
				ensure!(!Self::check_duplicate_cannabinoid(&id), Error::<T>::ItemAlreadyExists);
				let count = CannabinoidCount::<T>::get().unwrap_or(0);
				let id1 = id.clone();
				let name1 = name.clone();
				let description1 = description.clone();
				Cannabinoids::<T>::insert(id, Cannabinoid {
					id,
					name,
					description,
					products: Vec::new(),
				});
				CannabinoidByCount::<T>::insert(count.clone(), Cannabinoid {
					id: id1,
					name: name1,
					description: description1,
					products: Vec::new(),
				});
				CannabinoidCount::<T>::put(count + 1);
				Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,2))]
		pub fn new_user(
			origin: OriginFor<T>, 
			fname: Vec<u8>, 
			lname: Vec<u8>, 
			phone: Vec<u8>, 
			email: Vec<u8>,
			handle: Vec<u8>,
			handle_id: u128,
			bio: Vec<u8>,
			website: Vec<u8>,
			profile_image: Vec<u8>) -> DispatchResult {
				let who = ensure_signed(origin)?;
				ensure!(!Self::check_duplicate_user(&who), Error::<T>::UserAlreadyExists);
				ensure!(!Self::check_is_user(&who), Error::<T>::InsufficientPriv);
				
				let count = UserCount::<T>::get().unwrap_or(0);
				Users::<T>::insert(who.clone(), User {
					address: who.clone(),
					fname,
					lname,
					phone,
					email,
					handle,
					handle_id,
					bio,
					website,
					profile_image,
					total_orders: 0,
				});
				UserHandleAvailability::<T>::insert(handle_id, true);
				UserCount::<T>::put(count + 1);
				Ok(())
			}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,2))]
		pub fn edit_user(
			origin: OriginFor<T>, 
			fname: Vec<u8>, 
			lname: Vec<u8>, 
			phone: Vec<u8>, 
			email: Vec<u8>,
			handle: Vec<u8>,
			bio: Vec<u8>,
			website: Vec<u8>,
			handle_id: u128,
			profile_image: Vec<u8>,
			total_orders: u32) -> DispatchResult {
				let who = ensure_signed(origin)?;
				ensure!(!Self::check_is_user(&who), Error::<T>::InsufficientPriv);				
				Users::<T>::insert(who.clone(), User {
					address: who.clone(),
					fname,
					lname,
					phone,
					email,
					handle,
					handle_id,
					bio,
					website,
					profile_image,
					total_orders,
				});
				UserHandleAvailability::<T>::insert(handle_id, true);
				Ok(())
			}
		
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3,3))]
		pub fn purchase(
			origin: OriginFor<T>, 
			products: Vec<(Vec<u8>, u32, u32)>,
			date: Vec<u8>) -> DispatchResult {
				let who = ensure_signed(origin.clone())?;
				let total = Self::get_purchase_total(&products);
			//	pallet_balances::Pallet::<T>::transfer(origin, Owner::<T>::get() as <<T as frame_system::Config>::Lookup as sp_runtime::traits::StaticLookup>::Source, total as T::Balance);
				let count = OrderCount::<T>::get().unwrap_or(0);
				Orders::<T>::insert(count.clone() as u128, Order {
					id: count.clone(),
					user: who.clone(),
					products,
					total,
					date,
				});
				let order = Orders::<T>::get(count.clone()).unwrap_or(Default::default());
				let mut orders = OrdersByUser::<T>::get(who.clone()).unwrap_or(Default::default());
				orders.push(order);
				let mut user = Self::get_user(who.clone());
				user.total_orders += 1;
				Users::<T>::insert(who.clone(), user);
				OrdersByUser::<T>::insert(who.clone(), orders);
				OrderCount::<T>::put(count + 1);
				Ok(())
			}
	}

	impl<T: Config> Pallet<T> {
		//helpers
		fn production_cost_calc(amino_chain: &Vec<AminoAcid>) -> (u32, u32) {
			let mut total: u32 = 0;
			let mut yld: f64 = 0.97;
			for amino in amino_chain {
				total += total + amino.cost;
				yld = yld * 0.97
			}
			yld = yld / 0.97;
			yld = yld * 100.0;
			(total, yld as u32)
		}

		fn add_product_to_terpene(id: &u128, terpenes: &Vec<(u128, Vec<u8>, u32)>) {
			for t in terpenes {
				let mut terp = Terpenes::<T>::get(t.0).unwrap_or(Default::default());
				terp.products.push((*id, t.2));
				Terpenes::<T>::insert(t.0, terp);
			}
		}

		fn add_product_to_cannabinoid(id: &u128, cannabinoids: &Vec<(u128, Vec<u8>, u32)>) {
			for c in cannabinoids {
				let mut cann = Cannabinoids::<T>::get(c.0).unwrap_or(Default::default());
				cann.products.push((*id, c.2));
				Cannabinoids::<T>::insert(c.0, cann);
			}
		}

		fn get_purchase_total(products: &Vec<(Vec<u8>, u32, u32)>) -> u32 {
			let mut total: u32 = 0;
			for i in products {
				total += i.1 * i.2;
			}
			total
		}

		fn check_duplicate_peptide(id: &u128) -> bool {
			let peptide = Self::get_peptide(id);
			if peptide.0.name.len() > 0 {
				true
			} else {
				false
			}
		}

		fn check_duplicate_amino(id: &u128) -> bool {
			let amino = Self::get_amino(id);
			if amino.name.len() > 0 {
				true
			} else {
				false
			}
		}

		fn check_duplicate_terpene(id: &u128) -> bool {
			let terpene = Self::get_terpene(id).unwrap_or(Default::default());
			if terpene.name.len() > 0 {
				true
			} else {
				false
			}
		}

		fn check_duplicate_cannabinoid(id: &u128) -> bool {
			let cannabinoid = Self::get_cannabinoid(id).unwrap_or(Default::default());
			if cannabinoid.name.len() > 0 {
				true
			} else {
				false
			}
		}

		fn check_duplicate_cannabis(id: &u128) -> bool {
			let cannabis = Self::get_cannabis_product(id).unwrap_or(Default::default());
			if cannabis.name.len() > 0 {
				true
			} else {
				false
			}
		}

		fn check_duplicate_user(id: &T::AccountId) -> bool {
			let user = Self::get_user(id);
			if user.fname.len() > 0 {
				true
			} else {
				false
			}
		}

		fn check_duplicate_handle(id: &u128) -> bool {
			UserHandleAvailability::<T>::get(id).unwrap_or(false)
		}

		fn check_is_user(id: &T::AccountId) -> bool {
			let user = Users::<T>::get(id);
			if user.address.eq(id) {
				true
			} else {
				false
			}
		}
	}
}