#![cfg_attr(not(feature = "std"), no_std)]
// proof of existence

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*
    };
	use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

	/// 通过指定pallet依赖的参数和类型来配置pallet
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

    /// 定义Pallet来承载功能模块
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// 定义存储单元
	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	pub type Proofs<T:Config> = StorageMap<
        _,
        Blake2_128Concat,
        Vec<u8>,
        (T::AccountId, T::BlockNumber)
        >;

	/// 定义event
	#[pallet::event]
    // 将'T::AccountId'这样的元数据 转化为 'AccountId'这样能被客户端识别的类型
    #[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
        ClaimCreated(T::AccountId, Vec<u8>),
        ClaimRevoked(T::AccountId, Vec<u8>),
	}

	/// 定义error
	#[pallet::error]
	pub enum Error<T> {
        ProofAlreadyExist,
        ClaimNotExist,
        NotClaimOwner,
	}

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
            // 是否存在这个存证
			let sender = ensure_signed(origin)?;

			// 保证存证记录里没有这个存证，否侧返回错误
            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

			// 存储
			Proofs::<T>::insert(
                &claim, 
                (sender.clone(), frame_system::Pallet::<T>::block_number())
            );

			// 触发事件
			Self::deposit_event(Event::ClaimCreated(sender, claim));
			
            // DispatchResultWithPostInfo 是Option
			Ok(().into())
		}

        #[pallet::weight(0)]
		pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
            // 是否存在这个存证
			let sender = ensure_signed(origin)?;
            let (owner,_) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
            // 验证当前交易方是不是proof的owner
            ensure!(owner==sender, Error::<T>::NotClaimOwner);

            Proofs::<T>::remove(&claim);
           
			// 触发事件
			Self::deposit_event(Event::ClaimRevoked(sender, claim));
			
            // DispatchResultWithPostInfo 是Option
			Ok(().into())
		}
		
	}
}
