use super::framework::{XCVMContracts, XCVMDeploymentEvents, XCVMState};
use crate::tests::framework::{BlockchainTransaction, TestVM};
use cosmwasm_orchestrate::vm::Account;
use cosmwasm_std::{Attribute, BlockInfo, Event, MessageInfo, Timestamp};
use cosmwasm_vm::system::CUSTOM_CONTRACT_EVENT_PREFIX;
use cw20::{Cw20Coin, MinterResponse};
use cw_xcvm_asset_registry::contract::XCVM_ASSET_REGISTRY_EVENT_PREFIX;
use cw_xcvm_gateway::contract::XCVM_GATEWAY_EVENT_PREFIX;
use cw_xcvm_router::contract::XCVM_ROUTER_EVENT_PREFIX;
use xcvm_core::{Asset, AssetSymbol, Network, Picasso, ETH, PICA, USDC, USDT};

fn load_contracts() -> XCVMContracts {
	let code_cw20 = std::fs::read(std::env::var("CW20").unwrap()).unwrap();
	let code_asset_registry =
		std::fs::read(std::env::var("CW_XCVM_ASSET_REGISTRY").unwrap()).unwrap();
	let code_interpreter = std::fs::read(std::env::var("CW_XCVM_INTERPRETER").unwrap()).unwrap();
	let code_router = std::fs::read(std::env::var("CW_XCVM_ROUTER").unwrap()).unwrap();
	let code_gateway = std::fs::read(std::env::var("CW_XCVM_GATEWAY").unwrap()).unwrap();
	XCVMContracts::new(code_asset_registry, code_interpreter, code_router, code_gateway, code_cw20)
}

fn create_vm<N: Network>() -> TestVM<()> {
	let contracts = load_contracts();
	TestVM::new::<N>(contracts)
}

fn create_xcvm_vm<N: Network, T>(
	tx: BlockchainTransaction,
) -> (TestVM<XCVMState<T>>, XCVMDeploymentEvents) {
	create_vm::<N>()
		.deploy_xcvm::<T>(tx)
		.expect("Must be able to deploy XCVM contracts.")
}

fn find_event<'a>(
	mut events: impl Iterator<Item = &'a Event>,
	ty: impl Into<String>,
) -> Option<&'a Event> {
	let ty = ty.into();
	events.find(|x| x.ty == ty)
}

fn find_attr<'a>(
	mut attrs: impl Iterator<Item = &'a Attribute>,
	key: impl Into<String>,
) -> Option<&'a Attribute> {
	let key = key.into();
	attrs.find(|x| x.key == key)
}

fn xcvm_assert_prefixed_event<'a>(
	events: impl Iterator<Item = &'a Event>,
	ty: &str,
	key: &str,
	value: &str,
) {
	xcvm_assert_event(events, &format!("{CUSTOM_CONTRACT_EVENT_PREFIX}{ty}"), key, value);
}

fn xcvm_assert_event<'a>(
	events: impl Iterator<Item = &'a Event>,
	ty: &str,
	key: &str,
	value: &str,
) {
	let event = find_event(events, format!("{ty}"))
		.expect(&format!("XCVM contract must yield a {ty} event"));

	let attr = find_attr(event.attributes.iter(), key)
		.expect(&format!("XCVM {ty} event must contains a {key} attribute"));

	assert_eq!(attr.value, value);
}

#[test]
fn test_deploy() {
	let alice = Account::unchecked("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");

	let (_, events) = create_vm::<Picasso>()
		.deploy_xcvm::<()>(BlockchainTransaction {
			block: BlockInfo {
				height: 1000,
				time: Timestamp::from_seconds(0),
				chain_id: "PICASSO-MEMNET".into(),
			},
			transaction: None,
			info: MessageInfo { sender: alice.into(), funds: Default::default() },
			gas: 1_000_000,
		})
		.expect("Must be able to deploy XCVM contracts.");

	assert_eq!(events.registry_data, None);
	assert_eq!(events.gateway_data, None);

	xcvm_assert_prefixed_event(
		events.registry_events.iter(),
		XCVM_ASSET_REGISTRY_EVENT_PREFIX,
		"action",
		"instantiated",
	);

	// The gateway must deploy the router.
	xcvm_assert_prefixed_event(
		events.gateway_events.iter(),
		XCVM_ROUTER_EVENT_PREFIX,
		"action",
		"instantiated",
	);

	xcvm_assert_prefixed_event(
		events.gateway_events.iter(),
		XCVM_GATEWAY_EVENT_PREFIX,
		"action",
		"instantiated",
	);
}

fn xcvm_deploy_asset<A: Asset + AssetSymbol, T>(
	vm: TestVM<XCVMState<T>>,
	tx: BlockchainTransaction,
	initial_balances: Vec<Cw20Coin>,
	mint: Option<MinterResponse>,
) -> TestVM<XCVMState<T>> {
  let symbol = A::SYMBOL;
	let (vm, events) = vm.deploy_asset::<A>(tx, initial_balances, mint).expect(&format!("Must be able to instantiate and register {symbol} asset"));
  assert_eq!(events.registry_data, None);
	vm
}

#[test]
fn test_deploy_and_register_assets() {
	let alice = Account::unchecked("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");

	let tx = BlockchainTransaction {
		block: BlockInfo {
			height: 1000,
			time: Timestamp::from_seconds(0),
			chain_id: "PICASSO-MEMNET".into(),
		},
		transaction: None,
		info: MessageInfo { sender: alice.into(), funds: Default::default() },
		gas: 1_000_000,
	};

	let (vm, _) = create_xcvm_vm::<Picasso, ()>(tx.clone());

  let vm = xcvm_deploy_asset::<PICA, _>(vm, tx.clone(), Default::default(), None);
  let vm = xcvm_deploy_asset::<ETH, _>(vm, tx.clone(), Default::default(), None);
  let vm = xcvm_deploy_asset::<USDT, _>(vm, tx.clone(), Default::default(), None);
  xcvm_deploy_asset::<USDC, _>(vm, tx, Default::default(), None);
}
