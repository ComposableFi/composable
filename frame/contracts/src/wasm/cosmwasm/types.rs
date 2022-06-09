use alloc::string::String;
use serde::{de, ser, Deserialize, Deserializer, Serialize};
use sp_runtime::DispatchError;
use sp_std::vec::Vec;

use super::ComposableMsg;

pub mod read_limits {
	/// A mibi (mega binary)
	const MI: usize = 1024 * 1024;
	/// Max length (in bytes) of the result data from an instantiate call.
	pub const RESULT_INSTANTIATE: usize = 64 * MI;
	/// Max length (in bytes) of the result data from an execute call.
	pub const RESULT_EXECUTE: usize = 64 * MI;
	/// Max length (in bytes) of the result data from a migrate call.
	pub const RESULT_MIGRATE: usize = 64 * MI;
	/// Max length (in bytes) of the result data from a sudo call.
	pub const RESULT_SUDO: usize = 64 * MI;
	/// Max length (in bytes) of the result data from a reply call.
	pub const RESULT_REPLY: usize = 64 * MI;
	/// Max length (in bytes) of the result data from a query call.
	pub const RESULT_QUERY: usize = 64 * MI;
	/// Max length (in bytes) of the query data from a query_chain call.
	pub const REQUEST_QUERY: usize = 64 * MI;
}

/// The limits for the JSON deserialization.
///
/// Those limits are not used when the Rust JSON deserializer is bypassed by using the
/// public `call_*_raw` functions directly.
pub mod deserialization_limits {
	/// A kibi (kilo binary)
	const KI: usize = 1024;
	/// Max length (in bytes) of the result data from an instantiate call.
	pub const RESULT_INSTANTIATE: usize = 256 * KI;
	/// Max length (in bytes) of the result data from an execute call.
	pub const RESULT_EXECUTE: usize = 256 * KI;
	/// Max length (in bytes) of the result data from a migrate call.
	pub const RESULT_MIGRATE: usize = 256 * KI;
	/// Max length (in bytes) of the result data from a sudo call.
	pub const RESULT_SUDO: usize = 256 * KI;
	/// Max length (in bytes) of the result data from a reply call.
	pub const RESULT_REPLY: usize = 256 * KI;
	/// Max length (in bytes) of the result data from a query call.
	pub const RESULT_QUERY: usize = 256 * KI;
	/// Max length (in bytes) of the query data from a query_chain call.
	pub const REQUEST_QUERY: usize = 256 * KI;
}

pub type CosmwasmExecutionResult = ContractResult<Response<ComposableMsg>>;
pub type CosmwasmQueryResult = ContractResult<QueryResponse>;
pub type CosmwasmReplyResult = ContractResult<Response<ComposableMsg>>;

pub type QueryResponse = Binary;

pub trait DeserializeLimit {
	fn deserialize_limit() -> usize;
}

pub trait ReadLimit {
	fn read_limit() -> usize;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ReplyResult(pub CosmwasmExecutionResult);
impl DeserializeLimit for ReplyResult {
	fn deserialize_limit() -> usize {
		deserialization_limits::RESULT_REPLY
	}
}
impl ReadLimit for ReplyResult {
	fn read_limit() -> usize {
		read_limits::RESULT_REPLY
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct QueryResult(pub CosmwasmQueryResult);
impl DeserializeLimit for QueryResult {
	fn deserialize_limit() -> usize {
		deserialization_limits::RESULT_QUERY
	}
}
impl ReadLimit for QueryResult {
	fn read_limit() -> usize {
		read_limits::RESULT_QUERY
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ExecuteResult(pub CosmwasmExecutionResult);
impl DeserializeLimit for ExecuteResult {
	fn deserialize_limit() -> usize {
		deserialization_limits::RESULT_EXECUTE
	}
}
impl ReadLimit for ExecuteResult {
	fn read_limit() -> usize {
		read_limits::RESULT_EXECUTE
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateResult(pub CosmwasmExecutionResult);
impl DeserializeLimit for InstantiateResult {
	fn deserialize_limit() -> usize {
		deserialization_limits::RESULT_INSTANTIATE
	}
}
impl ReadLimit for InstantiateResult {
	fn read_limit() -> usize {
		read_limits::RESULT_INSTANTIATE
	}
}

#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryRequest<C = Empty> {
	Custom(C),
	Bank(BankQuery),
	Wasm(WasmQuery),
}

impl<C> DeserializeLimit for QueryRequest<C> {
	fn deserialize_limit() -> usize {
		deserialization_limits::REQUEST_QUERY
	}
}

impl<C> ReadLimit for QueryRequest<C> {
	fn read_limit() -> usize {
		read_limits::REQUEST_QUERY
	}
}

#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BankQuery {
	/// This calls into the native bank module for one denomination
	/// Return value is BalanceResponse
	Balance { address: String, denom: String },
	/// This calls into the native bank module for all denominations.
	/// Note that this may be much more expensive than Balance and should be avoided if possible.
	/// Return value is AllBalanceResponse.
	AllBalances { address: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct BalanceResponse {
	/// Always returns a Coin with the requested denom.
	/// This may be of 0 amount if no such funds.
	pub amount: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct AllBalanceResponse {
	/// Returns all non-zero coins held by this account.
	pub amount: Vec<Coin>,
}

#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WasmQuery {
	/// this queries the public API of another contract at a known address (with known ABI)
	/// Return value is whatever the contract returns (caller should know), wrapped in a
	/// ContractResult that is JSON encoded.
	Smart {
		contract_addr: String,
		/// msg is the json-encoded QueryMsg struct
		msg: Binary,
	},
	/// this queries the raw kv-store of the contract.
	/// returns the raw, unparsed data stored at that key, which may be an empty vector if not
	/// present
	Raw {
		contract_addr: String,
		/// Key is the raw key used in the contracts Storage
		key: Binary,
	},
	/// returns a ContractInfoResponse with metadata on the contract from the runtime
	ContractInfo { contract_addr: String },
}

#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContractInfoResponse {
	pub code_id: u64,
	/// address that instantiated this contract
	pub creator: String,
	/// admin who can run migrations (if any)
	pub admin: Option<String>,
	/// if set, the contract is pinned to the cache, and thus uses less gas when called
	pub pinned: bool,
	/// set if this contract has bound an IBC port
	pub ibc_port: Option<String>,
}

impl ContractInfoResponse {
	/// Convenience constructor for tests / mocks
	#[doc(hidden)]
	pub fn new(code_id: u64, creator: impl Into<String>) -> Self {
		Self { code_id, creator: creator.into(), admin: None, pinned: false, ibc_port: None }
	}
}

/// This is used for cases when we use ReplyOn::Never and the id doesn't matter
pub const UNUSED_MSG_ID: u64 = 0;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Response<T = Empty> {
	/// Optional list of messages to pass. These will be executed in order.
	/// If the ReplyOn variant matches the result (Always, Success on Ok, Error on Err),
	/// the runtime will invoke this contract's `reply` entry point
	/// after execution. Otherwise, they act like "fire and forget".
	/// Use `SubMsg::new` to create messages with the older "fire and forget" semantics.
	pub messages: Vec<SubMsg<T>>,
	/// The attributes that will be emitted as part of a "wasm" event.
	///
	/// More info about events (and their attributes) can be found in [*Cosmos SDK* docs].
	///
	/// [*Cosmos SDK* docs]: https://docs.cosmos.network/master/core/events.html
	pub attributes: Vec<Attribute>,
	/// Extra, custom events separate from the main `wasm` one. These will have
	/// `wasm-` prepended to the type.
	///
	/// More info about events can be found in [*Cosmos SDK* docs].
	///
	/// [*Cosmos SDK* docs]: https://docs.cosmos.network/master/core/events.html
	pub events: Vec<Event>,
	/// The binary payload to include in the response.
	pub data: Option<Binary>,
}

/// Use this to define when the contract gets a response callback.
/// If you only need it for errors or success you can select just those in order
/// to save gas.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReplyOn {
	/// Always perform a callback after SubMsg is processed
	Always,
	/// Only callback if SubMsg returned an error, no callback on success case
	Error,
	/// Only callback if SubMsg was successful, no callback on error case
	Success,
	/// Never make a callback - this is like the original CosmosMsg semantics
	Never,
}

/// A submessage that will guarantee a `reply` call on success or error, depending on
/// the `reply_on` setting. If you do not need to process the result, use regular messages instead.
///
/// Note: On error the submessage execution will revert any partial state changes due to this
/// message, but not revert any state changes in the calling contract. If this is required, it must
/// be done manually in the `reply` entry point.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubMsg<T = Empty> {
	/// An arbitrary ID chosen by the contract.
	/// This is typically used to match `Reply`s in the `reply` entry point to the submessage.
	pub id: u64,
	pub msg: CosmosMsg<T>,
	pub gas_limit: Option<u64>,
	pub reply_on: ReplyOn,
}

/// The result object returned to `reply`. We always get the ID from the submessage
/// back and then must handle success and error cases ourselves.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Reply {
	/// The ID that the contract set when emitting the `SubMsg`.
	/// Use this to identify which submessage triggered the `reply`.
	pub id: u64,
	pub result: SubMsgResult,
}

/// This is the result type that is returned from a sub message execution.
///
/// We use a custom type here instead of Rust's Result because we want to be able to
/// define the serialization, which is a public interface. Every language that compiles
/// to Wasm and runs in the ComsWasm VM needs to create the same JSON representation.
///
/// Until version 1.0.0-beta5, `ContractResult<SubMsgResponse>` was used instead
/// of this type. Once serialized, the two types are the same. However, in the Rust type
/// system we want different types for clarity and documenation reasons.
///
/// # Examples
///
/// Success:
///
/// ```
/// # use cosmwasm_std::{to_vec, Binary, Event, SubMsgResponse, SubMsgResult};
/// let response = SubMsgResponse {
///     data: Some(Binary::from_base64("MTIzCg==").unwrap()),
///     events: vec![Event::new("wasm").add_attribute("fo", "ba")],
/// };
/// let result: SubMsgResult = SubMsgResult::Ok(response);
/// assert_eq!(to_vec(&result).unwrap(), br#"{"ok":{"events":[{"type":"wasm","attributes":[{"key":"fo","value":"ba"}]}],"data":"MTIzCg=="}}"#);
/// ```
///
/// Failure:
///
/// ```
/// # use cosmwasm_std::{to_vec, SubMsgResult, Response};
/// let error_msg = String::from("Something went wrong");
/// let result = SubMsgResult::Err(error_msg);
/// assert_eq!(to_vec(&result).unwrap(), br#"{"error":"Something went wrong"}"#);
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubMsgResult {
	Ok(SubMsgResponse),
	/// An error type that every custom error created by contract developers can be converted to.
	/// This could potientially have more structure, but String is the easiest.
	#[serde(rename = "error")]
	Err(String),
}

impl From<SubMsgResult> for Result<SubMsgResponse, String> {
	fn from(original: SubMsgResult) -> Result<SubMsgResponse, String> {
		match original {
			SubMsgResult::Ok(value) => Ok(value),
			SubMsgResult::Err(err) => Err(err),
		}
	}
}

/// The information we get back from a successful sub message execution,
/// with full Cosmos SDK events.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubMsgResponse {
	pub events: Vec<Event>,
	pub data: Option<Binary>,
}

/// Like CustomQuery for better type clarity.
/// Also makes it shorter to use as a trait bound.
pub trait CustomMsg: Serialize + Clone + core::fmt::Debug + PartialEq {}

impl CustomMsg for Empty {}

#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
// See https://github.com/serde-rs/serde/issues/1296 why we cannot add De-Serialize trait bounds to T
pub enum CosmosMsg<T = Empty> {
	Bank(BankMsg),
	// by default we use RawMsg, but a contract can override that
	// to call into more app-specific code (whatever they define)
	Custom(T),
	Wasm(WasmMsg),
}

/// The message types of the bank module.
///
/// See https://github.com/cosmos/cosmos-sdk/blob/v0.40.0/proto/cosmos/bank/v1beta1/tx.proto
#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BankMsg {
	/// Sends native tokens from the contract to the given address.
	///
	/// This is translated to a [MsgSend](https://github.com/cosmos/cosmos-sdk/blob/v0.40.0/proto/cosmos/bank/v1beta1/tx.proto#L19-L28).
	/// `from_address` is automatically filled with the current contract's address.
	Send { to_address: String, amount: Vec<Coin> },
	/// This will burn the given coins from the contract's account.
	/// There is no Cosmos SDK message that performs this, but it can be done by calling the bank
	/// keeper. Important if a contract controls significant token supply that must be retired.
	Burn { amount: Vec<Coin> },
}

/// The message types of the wasm module.
///
/// See https://github.com/CosmWasm/wasmd/blob/v0.14.0/x/wasm/internal/types/tx.proto
#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WasmMsg {
	/// Dispatches a call to another contract at a known address (with known ABI).
	///
	/// This is translated to a [MsgExecuteContract](https://github.com/CosmWasm/wasmd/blob/v0.14.0/x/wasm/internal/types/tx.proto#L68-L78).
	/// `sender` is automatically filled with the current contract's address.
	Execute {
		contract_addr: String,
		/// msg is the json-encoded ExecuteMsg struct (as raw Binary)
		msg: Binary,
		funds: Vec<Coin>,
	},
	/// Instantiates a new contracts from previously uploaded Wasm code.
	///
	/// This is translated to a [MsgInstantiateContract](https://github.com/CosmWasm/wasmd/blob/v0.16.0-alpha1/x/wasm/internal/types/tx.proto#L47-L61).
	/// `sender` is automatically filled with the current contract's address.
	Instantiate {
		admin: Option<String>,
		code_id: u64,
		/// msg is the JSON-encoded InstantiateMsg struct (as raw Binary)
		msg: Binary,
		funds: Vec<Coin>,
		/// A human-readbale label for the contract
		label: String,
	},
	/// Migrates a given contracts to use new wasm code. Passes a MigrateMsg to allow us to
	/// customize behavior.
	///
	/// Only the contract admin (as defined in wasmd), if any, is able to make this call.
	///
	/// This is translated to a [MsgMigrateContract](https://github.com/CosmWasm/wasmd/blob/v0.14.0/x/wasm/internal/types/tx.proto#L86-L96).
	/// `sender` is automatically filled with the current contract's address.
	Migrate {
		contract_addr: String,
		/// the code_id of the new logic to place in the given contract
		new_code_id: u64,
		/// msg is the json-encoded MigrateMsg struct that will be passed to the new code
		msg: Binary,
	},
	/// Sets a new admin (for migrate) on the given contract.
	/// Fails if this contract is not currently admin of the target contract.
	UpdateAdmin { contract_addr: String, admin: String },
	/// Clears the admin on the given contract, so no more migration possible.
	/// Fails if this contract is not currently admin of the target contract.
	ClearAdmin { contract_addr: String },
}

/// A full [*Cosmos SDK* event].
///
/// This version uses string attributes (similar to [*Cosmos SDK* StringEvent]),
/// which then get magically converted to bytes for Tendermint somewhere between
/// the Rust-Go interface, JSON deserialization and the `NewEvent` call in Cosmos SDK.
///
/// [*Cosmos SDK* event]: https://docs.cosmos.network/master/core/events.html
/// [*Cosmos SDK* StringEvent]: https://github.com/cosmos/cosmos-sdk/blob/v0.42.5/proto/cosmos/base/abci/v1beta1/abci.proto#L56-L70
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Event {
	/// The event type. This is renamed to "ty" because "type" is reserved in Rust. This sucks, we
	/// know.
	#[serde(rename = "type")]
	pub ty: String,
	/// The attributes to be included in the event.
	///
	/// You can learn more about these from [*Cosmos SDK* docs].
	///
	/// [*Cosmos SDK* docs]: https://docs.cosmos.network/master/core/events.html
	pub attributes: Vec<Attribute>,
}

/// An key value pair that is used in the context of event attributes in logs
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Attribute {
	pub key: String,
	pub value: String,
}

impl<K: AsRef<str>, V: AsRef<str>> PartialEq<(K, V)> for Attribute {
	fn eq(&self, (k, v): &(K, V)) -> bool {
		(self.key.as_str(), self.value.as_str()) == (k.as_ref(), v.as_ref())
	}
}

impl<K: AsRef<str>, V: AsRef<str>> PartialEq<Attribute> for (K, V) {
	fn eq(&self, attr: &Attribute) -> bool {
		attr == self
	}
}

impl<K: AsRef<str>, V: AsRef<str>> PartialEq<(K, V)> for &Attribute {
	fn eq(&self, (k, v): &(K, V)) -> bool {
		(self.key.as_str(), self.value.as_str()) == (k.as_ref(), v.as_ref())
	}
}

impl<K: AsRef<str>, V: AsRef<str>> PartialEq<&Attribute> for (K, V) {
	fn eq(&self, attr: &&Attribute) -> bool {
		attr == self
	}
}

impl PartialEq<Attribute> for &Attribute {
	fn eq(&self, rhs: &Attribute) -> bool {
		*self == rhs
	}
}

impl PartialEq<&Attribute> for Attribute {
	fn eq(&self, rhs: &&Attribute) -> bool {
		self == *rhs
	}
}

/// An empty struct that serves as a placeholder in different places,
/// such as contracts that don't set a custom message.
///
/// It is designed to be expressable in correct JSON and JSON Schema but
/// contains no meaningful data. Previously we used enums without cases,
/// but those cannot represented as valid JSON Schema (https://github.com/CosmWasm/cosmwasm/issues/451)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Empty {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ContractResult<S> {
	Ok(S),
	/// An error type that every custom error created by contract developers can be converted to.
	/// This could potientially have more structure, but String is the easiest.
	#[serde(rename = "error")]
	Err(String),
}

// Implementations here mimic the Result API and should be implemented via a conversion to Result
// to ensure API consistency
impl<S> ContractResult<S> {
	/// Converts a `ContractResult<S>` to a `Result<S, String>` as a convenient way
	/// to access the full Result API.
	pub fn into_result(self) -> Result<S, String> {
		Result::<S, String>::from(self)
	}
}

impl<S> From<ContractResult<S>> for Result<S, String> {
	fn from(original: ContractResult<S>) -> Result<S, String> {
		match original {
			ContractResult::Ok(value) => Ok(value),
			ContractResult::Err(err) => Err(err),
		}
	}
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Coin {
	pub denom: String,
	#[serde(with = "string")]
	pub amount: u128,
}

impl core::fmt::Display for Coin {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		// We use the formatting without a space between amount and denom,
		// which is common in the Cosmos SDK ecosystem:
		// https://github.com/cosmos/cosmos-sdk/blob/v0.42.4/types/coin.go#L643-L645
		// For communication to end users, Coin needs to transformed anways (e.g. convert integer
		// uatom to decimal ATOM).
		write!(f, "{}{}", self.amount, self.denom)
	}
}

/// Binary is a wrapper around Vec<u8> to add base64 de/serialization
/// with serde. It also adds some helper methods to help encode inline.
///
/// This is only needed as serde-json-{core,wasm} has a horrible encoding for Vec<u8>
#[derive(Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Binary(pub Vec<u8>);

impl Binary {
	/// take an (untrusted) string and decode it into bytes.
	/// fails if it is not valid base64
	pub fn from_base64(encoded: &str) -> Result<Self, DispatchError> {
		let binary =
			base64::decode(&encoded).map_err(|_| DispatchError::Other("invalid base64"))?;
		Ok(Binary(binary))
	}

	/// encode to base64 string (guaranteed to be success as we control the data inside).
	/// this returns normalized form (with trailing = if needed)
	pub fn to_base64(&self) -> String {
		base64::encode(&self.0)
	}

	pub fn as_slice(&self) -> &[u8] {
		self.0.as_slice()
	}

	/// Copies content into fixed-sized array.
	/// The result type `A: ByteArray` is a workaround for
	/// the missing [const-generics](https://rust-lang.github.io/rfcs/2000-const-generics.html).
	/// `A` is a fixed-sized array like `[u8; 8]`.
	///
	/// ByteArray is implemented for `[u8; 0]` to `[u8; 64]`, such that
	/// we are limited to 64 bytes for now.
	///
	/// # Examples
	///
	/// Copy to array of explicit length
	///
	/// ```
	/// # use cosmwasm_std::Binary;
	/// let binary = Binary::from(&[0xfb, 0x1f, 0x37]);
	/// let array: [u8; 3] = binary.to_array().unwrap();
	/// assert_eq!(array, [0xfb, 0x1f, 0x37]);
	/// ```
	///
	/// Copy to integer
	///
	/// ```
	/// # use cosmwasm_std::Binary;
	/// let binary = Binary::from(&[0x8b, 0x67, 0x64, 0x84, 0xb5, 0xfb, 0x1f, 0x37]);
	/// let num = u64::from_be_bytes(binary.to_array().unwrap());
	/// assert_eq!(num, 10045108015024774967);
	/// ```
	pub fn to_array<const LENGTH: usize>(&self) -> Result<[u8; LENGTH], DispatchError> {
		if self.len() != LENGTH {
			return Err(DispatchError::Other("length mismatch"))
		}

		let mut out: [u8; LENGTH] = [0; LENGTH];
		out.copy_from_slice(&self.0);
		Ok(out)
	}
}

impl core::fmt::Display for Binary {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "{}", self.to_base64())
	}
}

impl core::fmt::Debug for Binary {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		// Use an output inspired by tuples (https://doc.rust-lang.org/std/fmt/struct.Formatter.html#method.debug_tuple)
		// but with a custom implementation to avoid the need for an intemediate hex string.
		write!(f, "Binary(")?;
		for byte in self.0.iter() {
			write!(f, "{:02x}", byte)?;
		}
		write!(f, ")")?;
		Ok(())
	}
}

impl From<&[u8]> for Binary {
	fn from(binary: &[u8]) -> Self {
		Self(binary.to_vec())
	}
}

/// Just like Vec<u8>, Binary is a smart pointer to [u8].
/// This implements `*binary` for us and allows us to
/// do `&*binary`, returning a `&[u8]` from a `&Binary`.
/// With [deref coercions](https://doc.rust-lang.org/1.22.1/book/first-edition/deref-coercions.html#deref-coercions),
/// this allows us to use `&binary` whenever a `&[u8]` is required.
impl core::ops::Deref for Binary {
	type Target = [u8];

	fn deref(&self) -> &Self::Target {
		self.as_slice()
	}
}

// Reference
impl<const LENGTH: usize> From<&[u8; LENGTH]> for Binary {
	fn from(source: &[u8; LENGTH]) -> Self {
		Self(source.to_vec())
	}
}

// Owned
impl<const LENGTH: usize> From<[u8; LENGTH]> for Binary {
	fn from(source: [u8; LENGTH]) -> Self {
		Self(source.into())
	}
}

impl From<Vec<u8>> for Binary {
	fn from(vec: Vec<u8>) -> Self {
		Self(vec)
	}
}

impl From<Binary> for Vec<u8> {
	fn from(original: Binary) -> Vec<u8> {
		original.0
	}
}

/// Implement `encoding::Binary == std::vec::Vec<u8>`
impl PartialEq<Vec<u8>> for Binary {
	fn eq(&self, rhs: &Vec<u8>) -> bool {
		// Use Vec<u8> == Vec<u8>
		self.0 == *rhs
	}
}

/// Implement `std::vec::Vec<u8> == encoding::Binary`
impl PartialEq<Binary> for Vec<u8> {
	fn eq(&self, rhs: &Binary) -> bool {
		// Use Vec<u8> == Vec<u8>
		*self == rhs.0
	}
}

/// Implement `Binary == &[u8]`
impl PartialEq<&[u8]> for Binary {
	fn eq(&self, rhs: &&[u8]) -> bool {
		// Use &[u8] == &[u8]
		self.as_slice() == *rhs
	}
}

/// Implement `&[u8] == Binary`
impl PartialEq<Binary> for &[u8] {
	fn eq(&self, rhs: &Binary) -> bool {
		// Use &[u8] == &[u8]
		*self == rhs.as_slice()
	}
}

/// Serializes as a base64 string
impl Serialize for Binary {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: ser::Serializer,
	{
		serializer.serialize_str(&self.to_base64())
	}
}

/// Deserializes as a base64 string
impl<'de> Deserialize<'de> for Binary {
	fn deserialize<D>(deserializer: D) -> Result<Binary, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_str(Base64Visitor)
	}
}

struct Base64Visitor;

impl<'de> de::Visitor<'de> for Base64Visitor {
	type Value = Binary;

	fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
		formatter.write_str("valid base64 encoded string")
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
	where
		E: de::Error,
	{
		match Binary::from_base64(v) {
			Ok(binary) => Ok(binary),
			Err(_) => Err(E::custom("")),
		}
	}
}

/// A human readable address.
///
/// In Cosmos, this is typically bech32 encoded. But for multi-chain smart contracts no
/// assumptions should be made other than being UTF-8 encoded and of reasonable length.
///
/// This type represents a validated address. It can be created in the following ways
/// 1. Use `Addr::unchecked(input)`
/// 2. Use `let checked: Addr = deps.api.addr_validate(input)?`
/// 3. Use `let checked: Addr = deps.api.addr_humanize(canonical_addr)?`
/// 4. Deserialize from JSON. This must only be done from JSON that was validated before
///    such as a contract's state. `Addr` must not be used in messages sent by the user
///    because this would result in unvalidated instances.
///
/// This type is immutable. If you really need to mutate it (Really? Are you sure?), create
/// a mutable copy using `let mut mutable = Addr::to_string()` and operate on that `String`
/// instance.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Addr(String);

impl Addr {
	/// Creates a new `Addr` instance from the given input without checking the validity
	/// of the input. Since `Addr` must always contain valid addresses, the caller is
	/// responsible for ensuring the input is valid.
	///
	/// Use this in cases where the address was validated before or in test code.
	/// If you see this in contract code, it should most likely be replaced with
	/// `let checked: Addr = deps.api.addr_humanize(canonical_addr)?`.
	///
	/// ## Examples
	///
	/// ```
	/// # use cosmwasm_std::{Addr};
	/// let address = Addr::unchecked("foobar");
	/// assert_eq!(address, "foobar");
	/// ```
	pub fn unchecked(input: impl Into<String>) -> Addr {
		Addr(input.into())
	}

	#[inline]
	pub fn as_str(&self) -> &str {
		self.0.as_str()
	}

	/// Returns the UTF-8 encoded address string as a byte array.
	///
	/// This is equivalent to `address.as_str().as_bytes()`.
	#[inline]
	pub fn as_bytes(&self) -> &[u8] {
		self.0.as_bytes()
	}

	/// Utility for explicit conversion to `String`.
	#[inline]
	pub fn into_string(self) -> String {
		self.0
	}
}

impl core::fmt::Display for Addr {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "{}", &self.0)
	}
}

impl AsRef<str> for Addr {
	#[inline]
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

/// Implement `Addr == &str`
impl PartialEq<&str> for Addr {
	fn eq(&self, rhs: &&str) -> bool {
		self.0 == *rhs
	}
}

/// Implement `&str == Addr`
impl PartialEq<Addr> for &str {
	fn eq(&self, rhs: &Addr) -> bool {
		*self == rhs.0
	}
}

/// Implement `Addr == String`
impl PartialEq<String> for Addr {
	fn eq(&self, rhs: &String) -> bool {
		&self.0 == rhs
	}
}

/// Implement `String == Addr`
impl PartialEq<Addr> for String {
	fn eq(&self, rhs: &Addr) -> bool {
		self == &rhs.0
	}
}

// Addr->String is a safe conversion.
// However, the opposite direction is unsafe and must not be implemented.

impl From<Addr> for String {
	fn from(addr: Addr) -> Self {
		addr.0
	}
}

impl From<&Addr> for String {
	fn from(addr: &Addr) -> Self {
		addr.0.clone()
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CanonicalAddr(pub Binary);

impl From<&[u8]> for CanonicalAddr {
	fn from(source: &[u8]) -> Self {
		Self(source.into())
	}
}

impl From<Vec<u8>> for CanonicalAddr {
	fn from(source: Vec<u8>) -> Self {
		Self(source.into())
	}
}

impl From<CanonicalAddr> for Vec<u8> {
	fn from(source: CanonicalAddr) -> Vec<u8> {
		source.0.into()
	}
}

/// Just like Vec<u8>, CanonicalAddr is a smart pointer to [u8].
/// This implements `*canonical_address` for us and allows us to
/// do `&*canonical_address`, returning a `&[u8]` from a `&CanonicalAddr`.
/// With [deref coercions](https://doc.rust-lang.org/1.22.1/book/first-edition/deref-coercions.html#deref-coercions),
/// this allows us to use `&canonical_address` whenever a `&[u8]` is required.
impl core::ops::Deref for CanonicalAddr {
	type Target = [u8];

	fn deref(&self) -> &Self::Target {
		self.as_slice()
	}
}

impl CanonicalAddr {
	pub fn as_slice(&self) -> &[u8] {
		self.0.as_slice()
	}
}

impl core::fmt::Display for CanonicalAddr {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		for byte in self.0.as_slice() {
			write!(f, "{:02X}", byte)?;
		}
		Ok(())
	}
}

mod string {
	use core::{fmt::Display, str::FromStr};

	use serde::{de, Deserialize, Deserializer, Serializer};

	pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
	where
		T: Display,
		S: Serializer,
	{
		serializer.collect_str(value)
	}

	pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
	where
		T: FromStr,
		T::Err: Display,
		D: Deserializer<'de>,
	{
		alloc::string::String::deserialize(deserializer)?
			.parse()
			.map_err(de::Error::custom)
	}
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(#[serde(with = "string")] pub u128);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Env {
	pub block: BlockInfo,
	/// Information on the transaction this message was executed in.
	/// The field is unset when the
	/// `MsgExecuteContract`/`MsgInstantiateContract`/`MsgMigrateContract` is not executed as part
	/// of a transaction.
	pub transaction: Option<TransactionInfo>,
	pub contract: ContractInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TransactionInfo {
	/// The position of this transaction in the block. The first
	/// transaction has index 0.
	///
	/// This allows you to get a unique transaction indentifier in this chain
	/// using the pair (`env.block.height`, `env.transaction.index`).
	pub index: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BlockInfo {
	/// The height of a block is the number of blocks preceding it in the blockchain.
	pub height: u64,
	/// Absolute time of the block creation in seconds since the UNIX epoch (00:00:00 on 1970-01-01
	/// UTC).
	///
	/// The source of this is the [BFT Time in Tendermint](https://github.com/tendermint/tendermint/blob/58dc1726/spec/consensus/bft-time.md),
	/// which has the same nanosecond precision as the `Timestamp` type.
	///
	/// # Examples
	///
	/// Using chrono:
	///
	/// ```
	/// # use cosmwasm_std::{Addr, BlockInfo, ContractInfo, Env, MessageInfo, Timestamp, TransactionInfo};
	/// # let env = Env {
	/// #     block: BlockInfo {
	/// #         height: 12_345,
	/// #         time: Timestamp::from_nanos(1_571_797_419_879_305_533),
	/// #         chain_id: "cosmos-testnet-14002".to_string(),
	/// #     },
	/// #     transaction: Some(TransactionInfo { index: 3 }),
	/// #     contract: ContractInfo {
	/// #         address: Addr::unchecked("contract"),
	/// #     },
	/// # };
	/// # extern crate chrono;
	/// use chrono::NaiveDateTime;
	/// let seconds = env.block.time.seconds();
	/// let nsecs = env.block.time.subsec_nanos();
	/// let dt = NaiveDateTime::from_timestamp(seconds as i64, nsecs as u32);
	/// ```
	///
	/// Creating a simple millisecond-precision timestamp (as used in JavaScript):
	///
	/// ```
	/// # use cosmwasm_std::{Addr, BlockInfo, ContractInfo, Env, MessageInfo, Timestamp, TransactionInfo};
	/// # let env = Env {
	/// #     block: BlockInfo {
	/// #         height: 12_345,
	/// #         time: Timestamp::from_nanos(1_571_797_419_879_305_533),
	/// #         chain_id: "cosmos-testnet-14002".to_string(),
	/// #     },
	/// #     transaction: Some(TransactionInfo { index: 3 }),
	/// #     contract: ContractInfo {
	/// #         address: Addr::unchecked("contract"),
	/// #     },
	/// # };
	/// let millis = env.block.time.nanos() / 1_000_000;
	/// ```
	pub time: Timestamp,
	pub chain_id: String,
}

/// Additional information from [MsgInstantiateContract] and [MsgExecuteContract], which is passed
/// along with the contract execution message into the `instantiate` and `execute` entry points.
///
/// It contains the essential info for authorization - identity of the call, and payment.
///
/// [MsgInstantiateContract]: https://github.com/CosmWasm/wasmd/blob/v0.15.0/x/wasm/internal/types/tx.proto#L47-L61
/// [MsgExecuteContract]: https://github.com/CosmWasm/wasmd/blob/v0.15.0/x/wasm/internal/types/tx.proto#L68-L78
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MessageInfo {
	/// The `sender` field from `MsgInstantiateContract` and `MsgExecuteContract`.
	/// You can think of this as the address that initiated the action (i.e. the message). What
	/// that means exactly heavily depends on the application.
	///
	/// The x/wasm module ensures that the sender address signed the transaction or
	/// is otherwise authorized to send the message.
	///
	/// Additional signers of the transaction that are either needed for other messages or contain
	/// unnecessary signatures are not propagated into the contract.
	pub sender: Addr,
	/// The funds that are sent to the contract as part of `MsgInstantiateContract`
	/// or `MsgExecuteContract`. The transfer is processed in bank before the contract
	/// is executed such that the new balance is visible during contract execution.
	pub funds: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContractInfo {
	pub address: Addr,
}

/// SystemError is used for errors inside the VM and is API friendly (i.e. serializable).
///
/// This is used on return values for Querier as a nested result: Result<StdResult<T>, SystemError>
/// The first wrap (SystemError) will trigger if the contract address doesn't exist,
/// the QueryRequest is malformated, etc. The second wrap will be an error message from
/// the contract itself.
///
/// Such errors are only created by the VM. The error type is defined in the standard library, to
/// ensure the contract understands the error format without creating a dependency on cosmwasm-vm.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SystemError {
	InvalidRequest {
		error: String,
		request: Binary,
	},
	InvalidResponse {
		error: String,
		response: Binary,
	},
	NoSuchContract {
		/// The address that was attempted to query
		addr: String,
	},
	Unknown {},
	UnsupportedRequest {
		kind: String,
	},
}

/// This is the outer result type returned by a querier to the contract.
///
/// We use a custom type here instead of Rust's Result because we want to be able to
/// define the serialization, which is a public interface. Every language that compiles
/// to Wasm and runs in the ComsWasm VM needs to create the same JSON representation.
///
/// # Examples
///
/// Success:
///
/// ```
/// # use cosmwasm_std::{to_vec, Binary, ContractResult, SystemResult};
/// let data = Binary::from(b"hello, world");
/// let result = SystemResult::Ok(ContractResult::Ok(data));
/// assert_eq!(to_vec(&result).unwrap(), br#"{"ok":{"ok":"aGVsbG8sIHdvcmxk"}}"#);
/// ```
///
/// Failure:
///
/// ```
/// # use cosmwasm_std::{to_vec, Binary, ContractResult, SystemResult, SystemError};
/// let error = SystemError::Unknown {};
/// let result: SystemResult<Binary> = SystemResult::Err(error);
/// assert_eq!(to_vec(&result).unwrap(), br#"{"error":{"unknown":{}}}"#);
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SystemResult<S> {
	Ok(S),
	#[serde(rename = "error")]
	Err(SystemError),
}

// Implementations here mimic the Result API and should be implemented via a conversion to Result
// to ensure API consistency
impl<S> SystemResult<S> {
	/// Converts a `ContractResult<S>` to a `Result<S, SystemError>` as a convenient way
	/// to access the full Result API.
	pub fn into_result(self) -> Result<S, SystemError> {
		Result::<S, SystemError>::from(self)
	}

	pub fn unwrap(self) -> S {
		self.into_result().unwrap()
	}
}

impl<S> From<Result<S, SystemError>> for SystemResult<S> {
	fn from(original: Result<S, SystemError>) -> SystemResult<S> {
		match original {
			Ok(value) => SystemResult::Ok(value),
			Err(err) => SystemResult::Err(err),
		}
	}
}

impl<S> From<SystemResult<S>> for Result<S, SystemError> {
	fn from(original: SystemResult<S>) -> Result<S, SystemError> {
		match original {
			SystemResult::Ok(value) => Ok(value),
			SystemResult::Err(err) => Err(err),
		}
	}
}
