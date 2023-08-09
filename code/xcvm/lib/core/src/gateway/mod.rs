pub mod config;

pub use config::*;

use crate::prelude::*;

use crate::{
	transport::ibc::XcMessageData, AssetId, CallOrigin, Displayed, Funds, InterpreterOrigin,
	NetworkId,
};

/// Prefix used for all events attached to gateway responses.
pub const EVENT_PREFIX: &str = "xcvm.gateway";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum ExecuteMsg {
	Config(ConfigSubMsg),

	/// Sent by the user to execute a program on their behalf.
	ExecuteProgram {
		/// Program to execute.
		execute_program: ExecuteProgramMsg,
		tip: Addr,
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
pub enum TestSubMsg {
	/// instantiates contract
	InstantiateContract {
		/// code of contract to instantiate
		code_id: u64,
		/// body of instantiate message 
		msg: serde_cw_value::Value,
	},
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
	pub assets: Funds<Displayed<u128>>,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema, QueryResponses))]
pub enum QueryMsg {
	/// Returns [`AssetReference`] for an asset with given id.
	#[cfg_attr(feature = "std", returns(GetAssetByIdResponse))]
	GetAssetById { asset_id: AssetId },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct GetAssetByIdResponse {
	pub asset: AssetItem,
}

#[cfg(test)]
mod tests {
	use crate::{
		gateway::{ExecuteMsg, ExecuteProgramMsg},
		generate_asset_id,
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
			tip: Addr::unchecked("centauri12smx2wdlyttvyzvzg54y2vnqwq2qjatescq89n"),
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
			tip: Addr::unchecked("centauri12smx2wdlyttvyzvzg54y2vnqwq2qjatescq89n"),
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
			tip: Addr::unchecked("centauri12smx2wdlyttvyzvzg54y2vnqwq2qjatescq89n"),
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
			tip: Addr::unchecked("osmo12smx2wdlyttvyzvzg54y2vnqwq2qjatescq89n"),
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
