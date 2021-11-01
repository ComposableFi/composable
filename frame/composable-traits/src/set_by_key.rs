pub trait SetByKey<Key, Value> {
	fn set(k: Key, value: Value) -> Result<(), Value>;
}
