use codec::{Decode, Encode, FullCodec, MaxEncodedLen};
use frame_support::{
	pallet_prelude::{OptionQuery, StorageMap, StorageValue},
	storage::types::QueryKindTrait,
	traits::{Get, StorageInstance},
	ReversibleStorageHasher, StorageHasher,
};
use scale_info::TypeInfo;

/// Fold over a storage, block per block.
pub trait FoldStorage<S, K, V> {
	/// Execute a step of the fold, this does mutate the fold state storage to keep track for the
	/// next execution. Must be executed once per block.
	fn step(
		initial_strategy: FoldStrategy,
		initial_state: S,
		f: impl Fn(S, K, V) -> S,
	) -> BlockFold<S, K>;
}

/// Folding strategies.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum FoldStrategy {
	/// Fold `number_of_elements` elements per block.
	Chunk { number_of_elements: u32 },
}

impl FoldStrategy {
	pub fn new_chunk(number_of_elements: u32) -> Self {
		Self::Chunk { number_of_elements }
	}
}

/// Block folding state, storing the intermediate/final state as well as the previously iterated
/// key.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum BlockFold<S, K> {
	/// Folding initialization. Iteration will start with the fist element.
	Init { strategy: FoldStrategy, state: S },
	/// Folding continuation. We hold the previous key to continue from there at next iteration.
	Cont { strategy: FoldStrategy, state: S, previous_key: K },
	/// Folding done. We hold the final state.
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
		// TODO(hussein-aitlahcen): initial, clear implementation. Nice first issue: improve
		// performance by staying in `mutate`.
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
				let r = iterator.try_fold(
					(0_u32, current_state),
					|(processed, state), (key, value)| {
						let next_state = f(state, key.clone(), value);
						/* NOTE(hussein-aitlahcen):
						   The above `iter_from` is expected a previous key, the iterator start AFTER the provided previous key.
						   This mean we need to actually process the current item before returning his key as `previous_key`, otherwise it would be skipped by the next iteration.
						   Do not refactor this by moving the above line inside the if.
						*/
						if processed == number_of_elements.saturating_sub(1) {
							Err((next_state, key))
						} else {
							Ok((processed + 1, next_state))
						}
					},
				);
				match r {
					Ok((_, next_state)) => BlockFold::Done { state: next_state },
					Err((next_state, previous_key)) =>
						BlockFold::Cont { strategy, state: next_state, previous_key },
				}
			},
		};
		match next_fold_state {
			x @ BlockFold::Done { .. } => {
				StorageValue::<P, BlockFold<S, IK>, OptionQuery>::set(None);
				x
			},
			x => {
				StorageValue::<P, BlockFold<S, IK>, OptionQuery>::set(Some(x.clone()));
				x
			},
		}
	}
}

impl<S, K> BlockFold<S, K> {
	pub fn new(strategy: FoldStrategy, state: S) -> Self {
		BlockFold::Init { strategy, state }
	}
}

#[cfg(all(test, feature = "std"))]
mod tests {
	use frame_support::Identity;
	use sp_io::TestExternalities;

	#[frame_support::storage_alias]
	type QueueStorageMap = StorageMap<Prefix, Identity, u64, u64>;

	/// based on tests from frame_support, but there is no such test to show off partial drain
	/// and docs do not tell that drain happens if you iterate element, not just by calling
	/// drain
	#[test]
	fn proves_partial_drain_possible() {
		TestExternalities::default().execute_with(|| {
			QueueStorageMap::insert(1, 1);
			QueueStorageMap::insert(2, 2);
			QueueStorageMap::insert(3, 3);
			QueueStorageMap::insert(4, 4);

			{
				let mut drain = QueueStorageMap::drain();
				assert!(drain.next().is_some());
				assert!(drain.next().is_some());
				drop(drain);
			}
			{
				let mut drain = QueueStorageMap::drain();
				assert!(drain.next().is_some());
				assert!(drain.next().is_some());
				assert_eq!(drain.next(), None);
			}
		});
	}
}
