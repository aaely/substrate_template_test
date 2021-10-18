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
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
	
	use sp_std::prelude::*;
	

	#[derive(Debug, Clone, PartialEq, Default, Encode, Decode)]
	pub struct Peptide<AccountId> {
		created_by: AccountId,
		id: u128,
		name: Vec<u8>,
		price: u32,
		inventory: u32,
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
		cannabinoids: Vec<(u128, u32)>,
		terpenes: Vec<(u128, u32)>,
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
	pub (super) type Terpenes<T> = StorageMap<_, Twox64Concat, u128, Terpene>;

	#[pallet::storage]
	pub (super) type Cannabinoids<T> = StorageMap<_, Twox64Concat, u128, Cannabinoid>;

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
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,2))]
		pub fn create_peptide(
			origin: OriginFor<T>, 
			name: Vec<u8>, 
			id: u128, 
			price: u32, 
			inventory: u32, 
			chain: Vec<AminoAcid>) -> DispatchResult {
				// Check that the extrinsic was signed and get the signer.
				// This function will return an error if the extrinsic is not signed.
				// https://substrate.dev/docs/en/knowledgebase/runtime/origin
				let who = ensure_signed(origin)?;
				let who1 = &who.clone();
				let who2 = who.clone();
				let count = PeptideCount::<T>::get().unwrap_or(0);
				let production_cost = Self::production_cost_calc(&chain).0;
				let production_yield = Self::production_cost_calc(&chain).1;
				let name1 = name.clone();
				let chain1 = chain.clone();
				// Update storage.
				Peptides::<T>::insert(id, (Peptide {
					created_by: who,
					id,
					name,
					price,
					inventory,
				}, PeptideProfile {
					peptide_ref: id.clone(),
					chain,
					production_cost,
					production_yield,
				}));

				PeptideByCount::<T>::insert(count.clone(), (Peptide {
					created_by: who1.clone(),
					id,
					name: name1,
					price,
					inventory,
				}, PeptideProfile {
					peptide_ref: id.clone(),
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
		
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn update_peptide_inventory(origin: OriginFor<T>, id: u128, inventory: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let mut peptide = Self::get_peptide(id);
			peptide.0.inventory = inventory;
			Self::deposit_event(Event::PeptideInventoryUpdate(peptide.clone()));
			Peptides::<T>::insert(id, peptide);
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,3))]
		pub fn create_amino(
			origin: OriginFor<T>, 
			name: Vec<u8>, 
			id: u128, 
			cost: u32) -> DispatchResult {
				let who = ensure_signed(origin)?;
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
		pub fn buy_products(origin: OriginFor<T>, total: u32, peptides: Vec<u128>, cannabis: Vec<u128>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let total_cost = Self::get_purchase_total(&peptides, &cannabis);
			ensure!(total >= total_cost, Error::<T>::InsufficientAmount);
			
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
			cannabinoids: Vec<(u128, u32)>, 
			terpenes: Vec<(u128, u32)>) -> DispatchResult {
				let who = ensure_signed(origin)?;
				Self::add_product_to_cannabinoid(&id, &cannabinoids);
				Self::add_product_to_terpene(&id, &terpenes);
				CannabisProducts::<T>::insert(id, CannabisProduct {
					id,
					name,
					price,
					category,
					inventory,
					image_hash,
					cannabinoids,
					terpenes,
				});
				Ok(())
		}
		
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,2))]
		pub fn new_terpene(
			origin: OriginFor<T>, 
			id: u128, 
			name: Vec<u8>, 
			description: Vec<u8>) -> DispatchResult {
				let who = ensure_signed(origin)?;
				Terpenes::<T>::insert(id, Terpene {
					id,
					name,
					description,
					products: Vec::new(),
				});
				Ok(())
		}
		
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,2))]
		pub fn new_cannabinoid(
			origin: OriginFor<T>, 
			id: u128, 
			name: Vec<u8>, 
			description: Vec<u8>) -> DispatchResult {
				let who = ensure_signed(origin)?;
				Cannabinoids::<T>::insert(id, Cannabinoid {
					id,
					name,
					description,
					products: Vec::new(),
				});
				Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		//helpers
		pub fn production_cost_calc(amino_chain: &Vec<AminoAcid>) -> (u32, u32) {
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

		pub fn add_product_to_terpene(id: &u128, terpenes: &Vec<(u128, u32)>) {
			for t in terpenes {
				let mut terp = Terpenes::<T>::get(t.0).unwrap_or(Default::default());
				terp.products.push((*id, t.1));
				Terpenes::<T>::insert(t.0, terp);
			}
		}

		pub fn add_product_to_cannabinoid(id: &u128, cannabinoids: &Vec<(u128, u32)>) {
			for c in cannabinoids {
				let mut cann = Cannabinoids::<T>::get(c.0).unwrap_or(Default::default());
				cann.products.push((*id, c.1));
				Cannabinoids::<T>::insert(c.0, cann);
			}
		}

		pub fn get_purchase_total(peptides: &Vec<u128>, cannabis: &Vec<u128>) -> u32 {
			let mut total: u32 = 0;
			for id in peptides {
				let pep = Self::get_peptide(id);
				total += pep.0.price;
			}
			for _id in cannabis {
				let cann = Self::get_cannabis_product(_id).unwrap_or(Default::default());
				total += cann.price;
			}
			total
		}

		
	}
}
