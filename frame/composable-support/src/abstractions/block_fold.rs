use codec::{Decode, Encode, FullCodec, MaxEncodedLen};
use frame_support::{
	pallet_prelude::{OptionQuery, StorageMap, StorageValue},
	storage::types::QueryKindTrait,
	traits::{Get, StorageInstance},
	ReversibleStorageHasher, StorageHasher,
};
use scale_info::TypeInfo;

pub trait FoldStorage<S, K, V> {
	fn step(
		initial_strategy: FoldStrategy,
		initial_state: S,
		f: impl Fn(S, K, V) -> S,
	) -> BlockFold<S, K>;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum FoldStrategy {
	Chunk { number_of_elements: u32 },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum BlockFold<S, K> {
	Init { strategy: FoldStrategy, state: S },
	Cont { strategy: FoldStrategy, state: S, previous_key: K },
	Done { state: S },
}

impl<P, S, IP, IH, IK, IV, IQ, IE, MV> FoldStorage<S, IK, IV>
	for (StorageValue<P, BlockFold<S, IK>, OptionQuery>, StorageMap<IP, IH, IK, IV, IQ, IE, MV>)
where
	P: StorageInstance,
	S: FullCodec + Clone + 'static,

	IP: StorageInstance,
	IH: StorageHasher + ReversibleStorageHasher,
	IK: FullCodec + Clone + 'static,
	IV: FullCodec,
	IE: Get<IQ::Query> + 'static,
	IQ: QueryKindTrait<IV, IE>,
	MV: Get<Option<u32>>,
{
	fn step(
		initial_strategy: FoldStrategy,
		initial_state: S,
		f: impl Fn(S, IK, IV) -> S,
	) -> BlockFold<S, IK> {
		let fold_state = StorageValue::<P, BlockFold<S, IK>, OptionQuery>::mutate(|x| match x {
			None => {
				let initial = BlockFold::Init { strategy: initial_strategy, state: initial_state };
				*x = Some(initial.clone());
				initial
			},
			Some(bf) => bf.clone(),
		});
		let (strategy, current_state, mut iterator) = match fold_state {
			BlockFold::Init { strategy, state } =>
				(strategy, state, StorageMap::<IP, IH, IK, IV, IQ, IE, MV>::iter()),
			BlockFold::Cont { strategy, state, previous_key } => (
				strategy,
				state,
				StorageMap::<IP, IH, IK, IV, IQ, IE, MV>::iter_from(StorageMap::<
					IP,
					IH,
					IK,
					IV,
					IQ,
					IE,
					MV,
				>::hashed_key_for(previous_key)),
			),
			d @ BlockFold::Done { .. } => return d,
		};
		let next_fold_state = match strategy {
			FoldStrategy::Chunk { number_of_elements } => {
				let r =
					iterator.try_fold((0u32, current_state), |(processed, state), (key, value)| {
						let next_state = f(state, key.clone(), value);
						if processed == number_of_elements.saturating_sub(1) {
							Err((next_state, key))
						} else {
							Ok((processed + 1, next_state))
						}
					});
				match r {
					Ok((_, next_state)) => BlockFold::Done { state: next_state.clone() },
					Err((next_state, previous_key)) => {
						BlockFold::Cont { strategy, state: next_state.clone(), previous_key }
					},
				}
			},
		};
		match next_fold_state.clone() {
			BlockFold::Done { .. } => {
				StorageValue::<P, BlockFold<S, IK>, OptionQuery>::set(None);
			},
			x => {
				StorageValue::<P, BlockFold<S, IK>, OptionQuery>::set(Some(x));
			},
		}
		next_fold_state
	}
}

impl<S, K> BlockFold<S, K> {
	pub fn new(strategy: FoldStrategy, state: S) -> Self {
		BlockFold::Init { strategy, state }
	}
}
