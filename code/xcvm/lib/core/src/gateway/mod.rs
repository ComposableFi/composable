pub mod config;
mod query;

pub use config::*;
pub use query::*;

use crate::{
	prelude::*, transport::ibc::XcMessageData, AssetId, CallOrigin, Funds, InterpreterOrigin,
	NetworkId,
};

/// Prefix used for all events attached to gateway responses.
pub const EVENT_PREFIX: &str = "xcvm.gateway";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, derive_more::From)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum ExecuteMsg {
	Config(ConfigSubMsg),

	/// Sent by the user to execute a program on their behalf.
	ExecuteProgram {
		/// Program to execute.
		execute_program: ExecuteProgramMsg,
		tip: String,
	},

	/// Request to execute a program on behalf of given user.
	///
	/// This can only be sent by trusted contract.  The message is
	ExecuteProgramPrivileged {
		/// The origin of the call.
		call_origin: CallOrigin,
		/// Program to execute.
		execute_program: ExecuteProgramMsg,

		tip: Addr,
	},

	/// Message sent from interpreter trying to spawn program on another
	/// network.
	BridgeForward(BridgeForwardMsg),

	/// simple permissionless message which produce xcvm program to test routes
	Shortcut(ShortcutSubMsg),

	/// executed by host as part of memo handling
	MessageHook(XcMessageData),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum ShortcutSubMsg {
	Transfer {
		/// assets from there
		asset_id: AssetId,
		amount: Uint128,
		/// target network, can hope over several networks
		/// if route is stored in state
		network: NetworkId,
		/// by default receiver is this
		receiver: Option<String>,
	},
}

/// Definition of a program to be executed including its context.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct ExecuteProgramMsg {
	/// The program salt.
	/// If JSON, than hex encoded non prefixed lower case string.
	#[serde(serialize_with = "hex::serialize", deserialize_with = "hex::deserialize")]
	#[cfg_attr(feature = "std", schemars(schema_with = "String::json_schema"))]
	pub salt: Vec<u8>,
	/// The program.
	pub program: crate::shared::XcProgram,
	/// Assets to fund the XCVM interpreter instance
	/// The interpreter is funded prior to execution
	pub assets: Funds<crate::shared::Displayed<u128>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct BridgeForwardMsg {
	pub interpreter_origin: InterpreterOrigin,
	/// target network
	pub to: NetworkId,
	pub msg: ExecuteProgramMsg,
}

/// Wrapper for interfacing with a gateway contract.
///
/// Provides convenience methods for querying the gateway and sending execute
/// messages to it.  Queries use [`cosmwasm_std::QuerierWrapper`] to make the
/// request and return immediately.  Execute requests on the other hand are
/// asynchronous and done by returning a [`cosmwasm_std::CosmosMsg`] which needs
/// to be added to a [`cosmwasm_std::Response`] object.
///
/// The object can be JSON-serialised as the address of the gateway.  Note that
/// since it’s serialised as [`cosmwasm_std::Addr`] it should not be part of
/// public API and only serialised in trusted objects where addresses don’t need
/// to be validated.
#[cfg(feature = "cosmwasm")]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Gateway {
	address: cosmwasm_std::Addr,
}

#[cfg(feature = "cosmwasm")]
impl Gateway {
	pub fn new(address: cosmwasm_std::Addr) -> Self {
		Self { address }
	}

	/// Validates gateway address and if it’s correct constructs a new object.
	///
	/// This is mostly a wrapper around CosmWasm address validation API.
	pub fn addr_validate(
		api: &dyn cosmwasm_std::Api,
		address: &str,
	) -> cosmwasm_std::StdResult<Self> {
		api.addr_validate(address).map(Self::new)
	}

	/// Returns gateway contract’s address as a String.
	pub fn address(&self) -> cosmwasm_std::Addr {
		self.address.clone()
	}

	/// Creates a CosmWasm message executing given message on the gateway.
	///
	/// The returned message must be added to Response to take effect.
	pub fn execute(
		&self,
		msg: impl Into<ExecuteMsg>,
	) -> cosmwasm_std::StdResult<cosmwasm_std::CosmosMsg> {
		self.execute_with_funds(msg, Vec::new())
	}

	/// Creates a CosmWasm message executing given message on the gateway with
	/// given funds attached.
	///
	/// The returned message must be added to Response to take effect.
	pub fn execute_with_funds(
		&self,
		msg: impl Into<ExecuteMsg>,
		funds: Vec<cosmwasm_std::Coin>,
	) -> cosmwasm_std::StdResult<cosmwasm_std::CosmosMsg> {
		cosmwasm_std::wasm_execute(self.address(), &msg.into(), funds)
			.map(cosmwasm_std::CosmosMsg::from)
	}

	/// Queries the gateway for definition of an asset with given id.
	pub fn get_asset_by_id(
		&self,
		querier: cosmwasm_std::QuerierWrapper,
		asset_id: AssetId,
	) -> cosmwasm_std::StdResult<AssetItem> {
		let query = QueryMsg::GetAssetById { asset_id };
		self.do_query::<GetAssetResponse>(querier, query).map(|response| response.asset)
	}

	pub fn get_exchange_by_id(
		&self,
		querier: cosmwasm_std::QuerierWrapper,
		exchange_id: crate::service::dex::ExchangeId,
	) -> cosmwasm_std::StdResult<crate::service::dex::ExchangeItem> {
		let query = QueryMsg::GetExchangeById { exchange_id };
		self.do_query::<GetExchangeResponse>(querier, query)
			.map(|response| response.exchange)
	}

	/// Queries the gateway for definition of an asset with given local
	/// reference.
	pub fn get_local_asset_by_reference(
		&self,
		querier: cosmwasm_std::QuerierWrapper,
		reference: AssetReference,
	) -> cosmwasm_std::StdResult<AssetItem> {
		let query = QueryMsg::GetLocalAssetByReference { reference };
		self.do_query::<GetAssetResponse>(querier, query).map(|response| response.asset)
	}

	/// Sends a query to the gateway contract.
	fn do_query<R: serde::de::DeserializeOwned>(
		&self,
		querier: cosmwasm_std::QuerierWrapper,
		query: QueryMsg,
	) -> cosmwasm_std::StdResult<R> {
		let query = cosmwasm_std::WasmQuery::Smart {
			contract_addr: self.address().into(),
			msg: cosmwasm_std::to_binary(&query)?,
		};
		querier.query(&query.into())
	}
}

#[cfg(test)]
mod tests {
	use crate::{
		gateway::{ExecuteMsg, ExecuteProgramMsg},
		generate_asset_id, generate_network_prefixed_id,
		prelude::*,
		shared::*,
		Instruction,
	};

	#[test]
	fn noop() {
		let program = ExecuteMsg::ExecuteProgram {
			execute_program: ExecuteProgramMsg {
				salt: b"noop".to_vec(),
				program: XcProgram { tag: b"noop".to_vec(), instructions: [].into() },
				assets: <_>::default(),
			},
			tip: String::from("centauri12smx2wdlyttvyzvzg54y2vnqwq2qjatescq89n"),
		};
		let program = serde_json_wasm::to_string(&program).expect("serde");
		let expected = serde_json_wasm::to_string(
			&serde_json_wasm::from_str::<ExecuteMsg>(
				r#"
					{
					"execute_program": {
						"execute_program": {
							"salt": "6e6f6f70",
							"program": {
							"tag": "6e6f6f70",
							"instructions": []
							},
							"assets": []
						},
						"tip": "centauri12smx2wdlyttvyzvzg54y2vnqwq2qjatescq89n"
						}
					}
				"#,
			)
			.unwrap(),
		)
		.unwrap();
		assert_eq!(program, expected)
	}

	#[test]
	fn noop_with_asset() {
		let pica_on_centauri = generate_asset_id(2.into(), 0, 1);
		let program = ExecuteMsg::ExecuteProgram {
			execute_program: ExecuteProgramMsg {
				salt: b"noop_with_asset".to_vec(),
				program: XcProgram { tag: b"noop_with_asset".to_vec(), instructions: [].into() },
				assets: vec![(pica_on_centauri, 1_000_000_000u128)].into(),
			},
			tip: String::from("centauri12smx2wdlyttvyzvzg54y2vnqwq2qjatescq89n"),
		};

		let program = serde_json_wasm::to_string(&program).expect("serde");
		let expected = serde_json_wasm::to_string(
			&serde_json_wasm::from_str::<ExecuteMsg>(
				r#"
					{
					"execute_program": {
						"execute_program": {
							"salt": "6e6f6f705f776974685f6173736574",
							"program": {
								"tag": "6e6f6f705f776974685f6173736574",
								"instructions": [
								]
							},
							"assets": [
								[ "158456325028528675187087900673", "1000000000"]
							]
						},
						"tip": "centauri12smx2wdlyttvyzvzg54y2vnqwq2qjatescq89n"
						}
					}
				"#,
			)
			.unwrap(),
		)
		.unwrap();
		assert_eq!(program, expected)
	}

	#[test]
	fn spawn_with_asset() {
		let pica_on_centauri = generate_asset_id(2.into(), 0, 1);
		let pica_on_osmosis = generate_asset_id(3.into(), 0, 1);
		let program = ExecuteMsg::ExecuteProgram {
			execute_program: ExecuteProgramMsg {
				salt: b"spawn_with_asset".to_vec(),
				program: XcProgram {
					tag: b"spawn_with_asset".to_vec(),
					instructions: [Instruction::Spawn {
						network: 3.into(),
						salt: b"spawn_with_asset".to_vec(),
						assets: vec![(pica_on_osmosis, 1_000_000_000u128)].into(),
						program: XcProgram {
							tag: b"spawn_with_asset".to_vec(),
							instructions: [].into(),
						},
					}]
					.into(),
				},
				assets: vec![(pica_on_centauri, 1_000_000_000u128)].into(),
			},
			tip: String::from("centauri12smx2wdlyttvyzvzg54y2vnqwq2qjatescq89n"),
		};

		let program = serde_json_wasm::to_string(&program).expect("serde");
		let expected = serde_json_wasm::to_string(
			&serde_json_wasm::from_str::<ExecuteMsg>(
				r#"
				{
					"execute_program": {
					  "execute_program": {
						"salt": "737061776e5f776974685f6173736574",
						"program": {
						  "tag": "737061776e5f776974685f6173736574",
						  "instructions": [
							{
							  "spawn": {
								"network": 3,
								"salt": "737061776e5f776974685f6173736574",
								"assets": [
								  [
									"237684487542793012780631851009",
									{
									  "amount": {
										"intercept": "1000000000",
										"slope": "0"
									  },
									  "is_unit": false
									}
								  ]
								],
								"program": {
								  "tag": "737061776e5f776974685f6173736574",
								  "instructions": []
								}
							  }
							}
						  ]
						},
						"assets": [
						  [
							"158456325028528675187087900673",
							"1000000000"
						  ]
						]
					  },
					  "tip": "centauri12smx2wdlyttvyzvzg54y2vnqwq2qjatescq89n"
					}
				  }
				"#,
			)
			.unwrap(),
		)
		.unwrap();
		assert_eq!(program, expected)
	}

	#[test]
	fn spawn_with_asset_and_transfer() {
		let pica_on_centauri = generate_asset_id(2.into(), 0, 1);
		let pica_on_osmosis = generate_asset_id(3.into(), 0, 1);

		let program = ExecuteMsg::ExecuteProgram {
			execute_program: ExecuteProgramMsg {
				salt: b"spawn_with_asset".to_vec(),
				program: XcProgram {
					tag: b"spawn_with_asset".to_vec(),
					instructions: [Instruction::Spawn {
						network: 3.into(),
						salt: b"spawn_with_asset".to_vec(),
						assets: vec![(pica_on_osmosis, 1_000_000_000u128)].into(),
						program: XcProgram {
							tag: b"spawn_with_asset".to_vec(),
							instructions: [XcInstruction::Transfer {
								to: crate::Destination::Account(
									Binary::from_base64("AB9vNpqXOevUvR5+JDnlljDbHhw=")
										.unwrap()
										.into(),
								),
								assets: XcFundsFilter::one(pica_on_osmosis, 1_000_000_000u128),
							}]
							.into(),
						},
					}]
					.into(),
				},
				assets: vec![(pica_on_centauri, 1_000_000_000u128)].into(),
			},
			tip: String::from("centauri12smx2wdlyttvyzvzg54y2vnqwq2qjatescq89n"),
		};

		let program = serde_json_wasm::to_string(&program).expect("serde");
		let expected = serde_json_wasm::to_string(
			&serde_json_wasm::from_str::<ExecuteMsg>(
				r#"
				{
					"execute_program": {
						"execute_program": {
							"salt": "737061776e5f776974685f6173736574",
							"program": {
								"tag": "737061776e5f776974685f6173736574",
								"instructions": [
									{
										"spawn": {
											"network": 3,
											"salt": "737061776e5f776974685f6173736574",
											"assets": [
												[
													"237684487542793012780631851009",
													{
														"amount": {
															"intercept": "1000000000",
															"slope": "0"
														},
														"is_unit": false
													}
												]
											],
											"program": {
												"tag": "737061776e5f776974685f6173736574",
												"instructions": [
													{
														"transfer": {
															"to": {
																"account": "AB9vNpqXOevUvR5+JDnlljDbHhw="
															},
															"assets": [
																[
																	"237684487542793012780631851009",
																	{
																		"amount": {
																			"intercept": "1000000000",
																			"slope": "0"
																		},
																		"is_unit": false
																	}
																]
															]
														}
													}
												]
											}
										}
									}
								]
							},
							"assets": [
								[
									"158456325028528675187087900673",
									"1000000000"
								]
							]
						},
						"tip": "centauri12smx2wdlyttvyzvzg54y2vnqwq2qjatescq89n"
					}
				}
				"#,
			)
			.unwrap(),
		)
		.unwrap();
		assert_eq!(program, expected)
	}

	#[test]
	fn spawn_with_asset_swap_and_transfer_back() {
		let pica_on_centauri = generate_asset_id(2.into(), 0, 1);
		let pica_on_osmosis = generate_asset_id(3.into(), 0, 1);
		let osmo_on_osmosis = generate_asset_id(3.into(), 0, 2);
		let osmo_on_centauri = generate_asset_id(2.into(), 0, 2);
		let pica_osmo_on_osmosis = generate_network_prefixed_id(3.into(), 100, 1);

		let program = ExecuteMsg::ExecuteProgram {
			execute_program: ExecuteProgramMsg {
				salt: b"spawn_with_asset".to_vec(),
				program: XcProgram {
					tag: b"spawn_with_asset".to_vec(),
					instructions: [Instruction::Spawn {
						network: 3.into(),
						salt: b"spawn_with_asset".to_vec(),
						assets: vec![(pica_on_osmosis, 1_000_000_000u128)].into(),
						program: XcProgram {
							tag: b"spawn_with_asset".to_vec(),
							instructions: [
								XcInstruction::Exchange {
									id: pica_osmo_on_osmosis.into(),
									give: XcFundsFilter::one(pica_on_osmosis, 1_000_000_000u128),
									want: XcFundsFilter::one(osmo_on_osmosis, 1_000u128),
								},
								XcInstruction::Spawn {
									network: 2.into(),
									salt: b"spawn_with_asset".to_vec(),
									assets: XcFundsFilter::one(osmo_on_centauri, (100, 100)),
									program: XcProgram {
										tag: b"spawn_with_asset".to_vec(),
										instructions: 
										[XcInstruction::Transfer {
											to: crate::Destination::Account(
												Binary::from_base64("AB9vNpqXOevUvR5+JDnlljDbHhw=")
													.unwrap()
													.into(),
											),
											assets: XcFundsFilter::one(osmo_on_centauri, (100, 100)),
										}].into(),
									},
								},
							]
							.into(),
						},
					}]
					.into(),
				},
				assets: vec![(pica_on_centauri, 1_000_000_000u128)].into(),
			},
			tip: String::from("centauri12smx2wdlyttvyzvzg54y2vnqwq2qjatescq89n"),
		};

		//pica_on_osmosis

		let program = serde_json_wasm::to_string(&program).expect("serde");
		let expected = serde_json_wasm::to_string(
			&serde_json_wasm::from_str::<ExecuteMsg>(
				r#"
				{
					"execute_program": {
						"execute_program": {
							"salt": "737061776e5f776974685f6173736574",
							"program": {
								"tag": "737061776e5f776974685f6173736574",
								"instructions": [
									{
										"spawn": {
											"network": 3,
											"salt": "737061776e5f776974685f6173736574",
											"assets": [
												[
													"237684487542793012780631851009",
													{
														"amount": {
															"intercept": "1000000000",
															"slope": "0"
														},
														"is_unit": false
													}
												]
											],
											"program": {
												"tag": "737061776e5f776974685f6173736574",
												"instructions": [
													{
														"exchange": {
															"id": "237684489387467420151587012609",
															"give": [
																[
																	"237684487542793012780631851009",
																	{
																		"amount": {
																			"intercept": "1000000000",
																			"slope": "0"
																		},
																		"is_unit": false
																	}
																]
															],
															"want": [
																[
																	"237684487542793012780631851010",
																	{
																		"amount": {
																			"intercept": "1000",
																			"slope": "0"
																		},
																		"is_unit": false
																	}
																]
															]
														}
													},
													{
														"spawn": {
															"network": 2,
															"salt": "737061776e5f776974685f6173736574",
															"assets": [
																[
																	"158456325028528675187087900674",
																	{
																		"amount": {
																			"intercept": "0",
																			"slope": "1000000000000000000"
																		},
																		"is_unit": false
																	}
																]
															],
															"program": {
																"tag": "737061776e5f776974685f6173736574",
																"instructions": [
																	{
																		"transfer": {
																			"to": {
																				"account": "AB9vNpqXOevUvR5+JDnlljDbHhw="
																			},
																			"assets": [
																				[
																					"158456325028528675187087900674",
																					{
																						"amount": {
																							"intercept": "0",
																							"slope": "1000000000000000000"
																						},
																						"is_unit": false
																					}
																				]
																			]
																		}
																	}
																]
															}
														}
													}
												]
											}
										}
									}
								]
							},
							"assets": [
								[
									"158456325028528675187087900673",
									"1000000000"
								]
							]
						},
						"tip": "centauri12smx2wdlyttvyzvzg54y2vnqwq2qjatescq89n"
					}
				}
				"#,
			)
			.unwrap(),
		)
		.unwrap();
		assert_eq!(program, expected)
	}

	#[test]
	fn osmosis_spawn_with_asset() {
		let osmo_on_osmosis = generate_asset_id(3.into(), 0, 1001);
		let osmo_on_centauri = generate_asset_id(2.into(), 0, 1001);
		let program: ExecuteMsg = ExecuteMsg::ExecuteProgram {
			execute_program: ExecuteProgramMsg {
				salt: b"spawn_with_asset".to_vec(),
				program: XcProgram {
					tag: b"spawn_with_asset".to_vec(),
					instructions: [Instruction::Spawn {
						network: 2.into(),
						salt: b"spawn_with_asset".to_vec(),
						assets: vec![(osmo_on_centauri, 1_000_000_000u128)].into(),
						program: XcProgram {
							tag: b"spawn_with_asset".to_vec(),
							instructions: [].into(),
						},
					}]
					.into(),
				},
				assets: vec![(osmo_on_osmosis, 1_000_000_000u128)].into(),
			},
			tip: String::from("osmo12smx2wdlyttvyzvzg54y2vnqwq2qjatescq89n"),
		};

		let program = serde_json_wasm::to_string(&program).expect("serde");
		//assert_eq!(program, "123");
		let expected = serde_json_wasm::to_string(
			&serde_json_wasm::from_str::<ExecuteMsg>(
				r#"
				{
					"execute_program": {
						"execute_program": {
							"salt": "737061776e5f776974685f6173736574",
							"program": {
								"tag": "737061776e5f776974685f6173736574",
								"instructions": [
									{
										"spawn": {
											"network": 2,
											"salt": "737061776e5f776974685f6173736574",
											"assets": [
												[
													"158456325028528675187087901673",
													{
														"amount": {
															"intercept": "1000000000",
															"slope": "0"
														},
														"is_unit": false
													}
												]
											],
											"program": {
												"tag": "737061776e5f776974685f6173736574",
												"instructions": []
											}
										}
									}
								]
							},
							"assets": [
								[
									"237684487542793012780631852009",
									"1000000000"
								]
							]
						},
						"tip": "osmo12smx2wdlyttvyzvzg54y2vnqwq2qjatescq89n"
					}
				}
				"#,
			)
			.unwrap(),
		)
		.unwrap();
		assert_eq!(program, expected)
	}
}
