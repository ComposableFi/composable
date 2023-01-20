#[cfg(test)]
mod tests;

#[cfg(test)]
mod tests1 {
	use cosmwasm_orchestrate::{
		cosmwasm_std::{
			Addr, Binary, BlockInfo, ContractInfo, Env, Event, IbcOrder, MessageInfo, Timestamp,
			TransactionInfo,
		},
		ibc::IbcNetwork,
		vm::{Account, State, SubstrateAddressHandler},
		Dispatch, StateBuilder, SubstrateApi,
	};
	use cosmwasm_std::{CanonicalAddr, Uint128};
	use cw20::{Cw20Coin, Cw20ExecuteMsg, Cw20QueryMsg, MinterResponse};
	use cw20_base::msg::InstantiateMsg;
	use cw_xcvm_asset_registry::msg::AssetReference;
	use xcvm_core::{
		Amount, Asset, BridgeSecurity, Destination, Funds, Juno, Network, NetworkId, Picasso,
		ProgramBuilder, PICA,
	};

	async fn setup_xcvm(
		network_id: NetworkId,
		block: BlockInfo,
		transaction: Option<TransactionInfo>,
		info: MessageInfo,
	) -> (Account, Account, Account, Account, State<(), SubstrateAddressHandler>) {
		let code_cw20 = std::fs::read(std::env::var("CW20").unwrap()).unwrap();
		let code_asset_registry =
			std::fs::read(std::env::var("CW_XCVM_ASSET_REGISTRY").unwrap()).unwrap();
		let code_interpreter =
			std::fs::read(std::env::var("CW_XCVM_INTERPRETER").unwrap()).unwrap();
		let code_router = std::fs::read(std::env::var("CW_XCVM_ROUTER").unwrap()).unwrap();
		let code_gateway = std::fs::read(std::env::var("CW_XCVM_GATEWAY").unwrap()).unwrap();
		let code_pingpong = std::fs::read(std::env::var("CW_XCVM_PINGPONG").unwrap()).unwrap();

		let mut state = StateBuilder::<SubstrateAddressHandler>::new()
			.add_codes(vec![
				&code_asset_registry,
				&code_interpreter,
				&code_router,
				&code_gateway,
				&code_cw20,
				&code_pingpong,
			])
			.build();

		log::debug!("{:?}", state);

		// XCVM registry deployment
		let (registry_address, _) = SubstrateApi::<Dispatch>::instantiate(
			&mut state,
			1,
			None,
			block.clone(),
			transaction.clone(),
			info.clone(),
			100_000_000_000,
			cw_xcvm_asset_registry::msg::InstantiateMsg {},
		)
		.unwrap();

		// XCVM gateway deployment
		let (gateway_address, (_, gateway_events)): (Account, (Option<Binary>, Vec<Event>)) =
			SubstrateApi::<Dispatch>::instantiate(
				&mut state,
				4,
				None,
				block.clone(),
				transaction.clone(),
				info.clone(),
				100_000_000_000,
				cw_xcvm_gateway::msg::InstantiateMsg {
					config: cw_xcvm_gateway::state::Config {
						registry_address: registry_address.clone().to_string(),
						router_code_id: 3,
						interpreter_code_id: 2,
						network_id,
						admin: info.sender.clone().into_string(),
					},
				},
			)
			.unwrap();

		let router_address: Account = gateway_events
			.iter()
			.find_map(|e| {
				if e.ty == format!("wasm-{}", cw_xcvm_router::contract::XCVM_ROUTER_EVENT_PREFIX) {
					e.attributes.iter().find(|a| a.key == "_contract_address")
				} else {
					None
				}
			})
			.expect("impossible")
			.value
			.clone()
			.try_into()
			.expect("impossible");

		// CW20 Pica address
		let (pica_address, _) = SubstrateApi::<Dispatch>::instantiate(
			&mut state,
			5,
			None,
			block.clone(),
			transaction.clone(),
			info.clone(),
			100_000_000_000,
			InstantiateMsg {
				name: "Picasso".into(),
				symbol: "PICA".into(),
				decimals: 12,
				initial_balances: vec![Cw20Coin {
					amount: 1_000_000_000_000_000_000u128.into(),
					address: info.sender.clone().into_string(),
				}],
				mint: Some(MinterResponse {
					minter: Addr::from(gateway_address.clone()).into_string(),
					cap: None,
				}),
				marketing: None,
			},
		)
		.unwrap();

		log::info!("{}", router_address);

		// Pingpong contract address
		let (pingpong_address, _) = SubstrateApi::<Dispatch>::instantiate(
			&mut state,
			6,
			None,
			block.clone(),
			transaction.clone(),
			info.clone(),
			100_000_000_000,
			cw_xcvm_pingpong::msg::InstantiateMsg {
				router_address: router_address.to_string(),
				network_id,
			},
		)
		.unwrap();

		SubstrateApi::<Dispatch>::execute(
			&mut state,
			Env {
				block: block.clone(),
				transaction: transaction.clone(),
				contract: ContractInfo { address: registry_address.into() },
			},
			MessageInfo { sender: info.sender.clone(), funds: Default::default() },
			1_000_000_000,
			cw_xcvm_asset_registry::msg::ExecuteMsg::RegisterAsset {
				asset_id: PICA::ID.into(),
				reference: AssetReference::Virtual { cw20_address: pica_address.clone().into() },
			},
		)
		.unwrap();

		(gateway_address, router_address, pica_address, pingpong_address, state)
	}

	#[tokio::test]
	async fn balanced_liquidity() {
		env_logger::init();

		let sender = Account::unchecked("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
		let network = Picasso::ID;
		let network_counterparty = Juno::ID;

		let block_info = BlockInfo {
			height: 1000,
			time: Timestamp::from_seconds(0),
			chain_id: "ibc:in-memory".into(),
		};
		let transaction_info = Option::<TransactionInfo>::None;
		let message_info = MessageInfo { sender: sender.clone().into(), funds: Default::default() };

		let (gateway, router, pica, _pingpong, mut state) =
			setup_xcvm(network, block_info.clone(), transaction_info.clone(), message_info.clone())
				.await;
		let (
			gateway_counterparty,
			_router_counterparty,
			pica_counterparty,
			_pingpong_counterparty,
			mut state_counterparty,
		) = setup_xcvm(
			network_counterparty,
			block_info.clone(),
			transaction_info.clone(),
			message_info.clone(),
		)
		.await;

		let env = Env {
			block: block_info.clone(),
			transaction: transaction_info.clone(),
			contract: ContractInfo { address: gateway.clone().into() },
		};
		let env_counterparty = Env {
			block: block_info.clone(),
			transaction: transaction_info.clone(),
			contract: ContractInfo { address: gateway_counterparty.clone().into() },
		};

		let relayer = Account::unchecked("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKtxzC");
		let relayer_counterpary =
			Account::unchecked("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKuT9N");

		let info = MessageInfo { sender: relayer.into(), funds: Default::default() };
		let info_counterparty =
			MessageInfo { sender: relayer_counterpary.into(), funds: Default::default() };

		let channel_id = "ibc:in-memory-0".to_string();
		let mut ibc_network =
			IbcNetwork::<(), SubstrateAddressHandler>::new(&mut state, &mut state_counterparty);
		let channel = ibc_network
			.handshake(
				channel_id.clone(),
				"xcvm".into(),
				IbcOrder::Unordered,
				"ibc:connection:memory".into(),
				env.clone(),
				env_counterparty.clone(),
				info.clone(),
				info_counterparty.clone(),
				1_000_000_000,
			)
			.unwrap();

		SubstrateApi::<Dispatch>::execute(
			ibc_network.state,
			Env {
				block: block_info.clone(),
				transaction: transaction_info.clone(),
				contract: ContractInfo { address: gateway.clone().into() },
			},
			MessageInfo { sender: sender.clone().into(), funds: Default::default() },
			1_000_000_000,
			cw_xcvm_common::gateway::ExecuteMsg::IbcSetNetworkChannel {
				network_id: network_counterparty,
				channel_id: channel_id.clone(),
			},
		)
		.unwrap();

		SubstrateApi::<Dispatch>::execute(
			ibc_network.state_counterparty,
			Env {
				block: block_info.clone(),
				transaction: transaction_info.clone(),
				contract: ContractInfo { address: gateway_counterparty.clone().into() },
			},
			MessageInfo { sender: sender.clone().into(), funds: Default::default() },
			1_000_000_000,
			cw_xcvm_common::gateway::ExecuteMsg::IbcSetNetworkChannel {
				network_id: network,
				channel_id: channel_id.clone(),
			},
		)
		.unwrap();

		let transfer_amount = 1_000_000_000_000u128;

		SubstrateApi::<Dispatch>::execute(
			ibc_network.state,
			Env {
				block: block_info.clone(),
				transaction: transaction_info.clone(),
				contract: ContractInfo { address: pica.clone().into() },
			},
			MessageInfo { sender: sender.clone().into(), funds: Default::default() },
			1_000_000_000,
			&Cw20ExecuteMsg::IncreaseAllowance {
				spender: Addr::from(router.clone()).into_string(),
				amount: transfer_amount.into(),
				expires: None,
			},
		)
		.unwrap();

		// SubstrateApi::<Dispatch>::execute(
		// 	ibc_network.state,
		// 	Env {
		// 		block: block_info.clone(),
		// 		transaction: transaction_info.clone(),
		// 		contract: ContractInfo { address: pingpong.clone().into() },
		// 	},
		// 	MessageInfo { sender: sender.clone().into(), funds: Default::default() },
		// 	1_000_000_000,
		// 	cw_xcvm_pingpong::msg::ExecuteMsg::Ping {
		// 		user_origin: UserOrigin {
		// 			network_id: network_counterparty,
		// 			user_id: pingpong_counterparty.0.as_bytes().to_vec().into(),
		// 		},
		// 		counter: 0,
		// 	},
		// )
		// .unwrap();

		let program = ProgramBuilder::<Picasso, CanonicalAddr, Funds>::new(vec![1, 2, 3])
			.spawn::<Juno, (), _, _>(
				*b"WHERE",
				*b"",
				BridgeSecurity::Deterministic,
				[(PICA::ID, Amount::absolute(1_000_000))],
				|juno_program| {
					juno_program.spawn::<Picasso, (), _, _>(
						*b"HOME",
						*b"",
						BridgeSecurity::Deterministic,
						[(PICA::ID, Amount::everything())],
						|picasso_program| {
							Ok(picasso_program
								.transfer(Destination::Relayer, [(PICA::ID, Amount::everything())]))
						},
					)
				},
			)
			.unwrap()
			.build();

		SubstrateApi::<Dispatch>::execute(
			ibc_network.state,
			Env {
				block: block_info.clone(),
				transaction: transaction_info.clone(),
				contract: ContractInfo { address: router.clone().into() },
			},
			MessageInfo { sender: sender.into(), funds: Default::default() },
			1_000_000_000,
			cw_xcvm_common::router::ExecuteMsg::ExecuteProgram {
				salt: Default::default(),
				program,
				assets: Default::default(),
			},
		)
		.unwrap();

		ibc_network
			.relay(
				channel,
				env,
				env_counterparty,
				info,
				info_counterparty,
				1_000_000_000,
				&(gateway, pica),
				&(gateway_counterparty, pica_counterparty),
				|_, _, _, _| {
					// Relay hook
				},
				|state,
				 state_counterparty,
				 (_gateway, pica),
				 (_gateway_counterparty, pica_counterparty)| {
					let juno_pica_supply = SubstrateApi::query::<_, cw20::TokenInfoResponse>(
						state,
						Env {
							block: block_info.clone(),
							transaction: transaction_info.clone(),
							contract: ContractInfo { address: pica.clone().into() },
						},
						&Cw20QueryMsg::TokenInfo {},
					)
					.unwrap();

					let picasso_pica_supply = SubstrateApi::query::<_, cw20::TokenInfoResponse>(
						state_counterparty,
						Env {
							block: block_info.clone(),
							transaction: transaction_info.clone(),
							contract: ContractInfo { address: pica_counterparty.clone().into() },
						},
						&Cw20QueryMsg::TokenInfo {},
					)
					.unwrap();

					assert_eq!(
						juno_pica_supply.total_supply + picasso_pica_supply.total_supply,
						Uint128::zero()
					);
				},
			)
			.unwrap();
	}
}
