use super::framework::{
	InMemoryIbcNetworkChannel, TestError, XCVMContracts, XCVMDeploymentEvents, XCVMState,
};
use crate::tests::framework::{BlockchainTransaction, TestVM};
use cosmwasm_orchestrate::vm::{Account, AddressHandler, SubstrateAddressHandler};
use cosmwasm_std::{
	Attribute, Binary, BlockInfo, CanonicalAddr, Event, IbcOrder, MessageInfo, Timestamp,
};
use cosmwasm_vm::system::CUSTOM_CONTRACT_EVENT_PREFIX;
use cw20::{Cw20Coin, Expiration, MinterResponse};
use cw_xcvm_asset_registry::contract::XCVM_ASSET_REGISTRY_EVENT_PREFIX;
use cw_xcvm_gateway::contract::{XCVM_GATEWAY_EVENT_PREFIX, XCVM_GATEWAY_IBC_VERSION};
use cw_xcvm_interpreter::contract::XCVM_INTERPRETER_EVENT_PREFIX;
use cw_xcvm_router::contract::XCVM_ROUTER_EVENT_PREFIX;
use cw_xcvm_utils::{DefaultXCVMProgram, Salt};
use proptest::proptest;
use std::assert_matches::assert_matches;
use xcvm_core::{
	Amount, Asset, AssetId, AssetSymbol, BridgeSecurity, Destination, Funds, Juno, Network,
	Picasso, ProgramBuilder, ETH, PICA, USDC, USDT,
};

#[macro_export]
macro_rules! assert_ok(
  ($result:expr) => {
    match $result {
      Ok(..) => {},
      Err(..) => assert!(false, "Expected Ok(..), Got {:?}", $result),
    }
  };
);

fn almost_eq(x: u128, y: u128, epsilon: u128) -> Result<(), (u128, u128)> {
	let delta = i128::abs(x as i128 - y as i128);
	if (delta as u128) <= epsilon {
		Ok(())
	} else {
		Err((x, y))
	}
}

fn mk_tx_raw(block: BlockInfo, sender: Account) -> BlockchainTransaction {
	BlockchainTransaction {
		block,
		transaction: None,
		info: MessageInfo { sender: sender.into(), funds: Default::default() },
		gas: u64::MAX,
	}
}

struct Disconnected;
struct Connected {
	network: InMemoryIbcNetworkChannel,
}
struct CrossChainScenario<T, S> {
	vm: TestVM<XCVMState<T>>,
	vm_counterparty: TestVM<XCVMState<T>>,
	events: XCVMDeploymentEvents,
	events_counterparty: XCVMDeploymentEvents,
	admin: Account,
	admin_counterparty: Account,
	block: BlockInfo,
	block_counterparty: BlockInfo,
	shared: S,
}

struct CrossChainDispatchResult {
	dispatch_data: Option<Binary>,
	dispatch_events: Vec<Event>,
	relay_data: Vec<Option<Binary>>,
	relay_events: Vec<Event>,
}

impl<T, S> CrossChainScenario<T, S> {
	fn set_block(mut self, height: u64) -> Self {
		self.block.height = height;
		self
	}

	fn advance_block(mut self, nb_of_blocks: u64) -> Self {
		self.block.height += nb_of_blocks;
		self
	}

	fn advance_time(mut self, nb_of_seconds: u64) -> Self {
		self.block.time = self.block.time.plus_seconds(nb_of_seconds);
		self
	}

	fn set_block_counterparty(mut self, height: u64) -> Self {
		self.block_counterparty.height = height;
		self
	}

	fn advance_block_counterparty(mut self, nb_of_blocks: u64) -> Self {
		self.block_counterparty.height += nb_of_blocks;
		self
	}

	fn advance_time_counterparty(mut self, nb_of_seconds: u64) -> Self {
		self.block_counterparty.time = self.block_counterparty.time.plus_seconds(nb_of_seconds);
		self
	}

	fn mk_tx(&self, sender: Account) -> BlockchainTransaction {
		mk_tx_raw(self.block.clone(), sender)
	}

	fn mk_tx_counterparty(&self, sender: Account) -> BlockchainTransaction {
		mk_tx_raw(self.block_counterparty.clone(), sender)
	}
}

impl<T, S> CrossChainScenario<T, S> {
	fn deploy_asset<A: Asset + AssetSymbol>(
		&mut self,
		initial_balances: impl IntoIterator<Item = Cw20Coin>,
	) {
		let tx = self.mk_tx(self.admin.clone());
		xcvm_deploy_asset::<A, T>(&mut self.vm, tx, initial_balances)
	}

	fn deploy_asset_counterparty<A: Asset + AssetSymbol>(
		&mut self,
		initial_balances: impl IntoIterator<Item = Cw20Coin>,
	) {
		let tx = self.mk_tx_counterparty(self.admin_counterparty.clone());
		xcvm_deploy_asset::<A, T>(&mut self.vm_counterparty, tx, initial_balances)
	}

	// fn crosschain(
	// 	&mut self,
	// 	relayer: Account,
	// 	relayer_counterparty: Account,
	// 	program_sender: Account,
	// 	program: DefaultXCVMProgram,
	// 	assets_to_transfer: &[(AssetId, u128)],
	// 	allowance_expiration: Option<Expiration>,
	// ) -> Result<(), TestError> {
	// 	let channel_id = "ibc:in-memory:picasso-juno";
	// 	let connection_id = "ibc:connection:0";
	// 	let version = XCVM_GATEWAY_IBC_VERSION;
	// 	let mut ibc_network = InMemoryIbcNetworkChannel::connect(
	// 		&mut self.vm,
	// 		&mut self.vm_counterparty,
	// 		channel_id,
	// 		connection_id,
	// 		version,
	// 		IbcOrder::Unordered,
	// 		mk_tx(relayer.clone()),
	// 		mk_tx(relayer_counterparty.clone()),
	// 		mk_tx(self.admin.clone()),
	// 		mk_tx(self.admin_counterparty.clone()),
	// 		u64::MAX,
	// 	)?;
	// 	let (data, events) = self.vm.dispatch_program_with_allowance(
	// 		mk_tx(program_sender.clone()),
	// 		[],
	// 		program,
	// 		assets_to_transfer.iter().copied().collect::<Vec<_>>(),
	// 		allowance_expiration,
	// 	)?;
	// 	ibc_network.relay(
	// 		&mut self.vm,
	// 		&mut self.vm_counterparty,
	// 		mk_tx(relayer.clone()),
	// 		mk_tx(relayer_counterparty.clone()),
	// 		u64::MAX,
	// 	)?;
	// 	assert_eq!(data, None);
	// 	xcvm_assert_prefixed_event(
	// 		events.iter(),
	// 		XCVM_ROUTER_EVENT_PREFIX,
	// 		"action",
	// 		"route.create",
	// 	);
	// 	xcvm_assert_prefixed_event(
	// 		events.iter(),
	// 		XCVM_ROUTER_EVENT_PREFIX,
	// 		"action",
	// 		"route.execute",
	// 	);
	// 	xcvm_assert_prefixed_event(
	// 		events.iter(),
	// 		XCVM_INTERPRETER_EVENT_PREFIX,
	// 		"action",
	// 		"execution.start",
	// 	);
	// 	xcvm_assert_prefixed_event(
	// 		events.iter(),
	// 		XCVM_INTERPRETER_EVENT_PREFIX,
	// 		"action",
	// 		"execution.success",
	// 	);
	// 	Ok(())
	// }
}

impl<T> CrossChainScenario<T, Disconnected> {
	fn new<N: Network, M: Network>(
		block: BlockInfo,
		block_counterparty: BlockInfo,
		admin: Account,
		admin_counterparty: Account,
	) -> Self {
		let (vm, events) = create_base_xcvm_vm::<N, T>(mk_tx_raw(block.clone(), admin.clone()));
		let (vm_counterparty, events_counterparty) = create_base_xcvm_vm::<M, T>(mk_tx_raw(
			block_counterparty.clone(),
			admin_counterparty.clone(),
		));
		Self {
			vm,
			vm_counterparty,
			events,
			events_counterparty,
			admin,
			admin_counterparty,
			block,
			block_counterparty,
			shared: Disconnected,
		}
	}

	fn connect(
		mut self,
		channel_id: impl Into<String>,
		connection_id: impl Into<String>,
		ordering: IbcOrder,
		relayer: Account,
		relayer_counterparty: Account,
	) -> Result<CrossChainScenario<T, Connected>, TestError> {
		let version = XCVM_GATEWAY_IBC_VERSION;
		let tx_relayer = self.mk_tx(relayer);
		let tx_relayer_counterparty = self.mk_tx_counterparty(relayer_counterparty);
		let tx_admin = self.mk_tx(self.admin.clone());
		let tx_admin_counterparty = self.mk_tx_counterparty(self.admin_counterparty.clone());
		let network = InMemoryIbcNetworkChannel::connect(
			&mut self.vm,
			&mut self.vm_counterparty,
			channel_id.into(),
			connection_id.into(),
			version,
			ordering,
			tx_relayer,
			tx_relayer_counterparty,
			tx_admin,
			tx_admin_counterparty,
			u64::MAX,
		)?;
		Ok(CrossChainScenario {
			vm: self.vm,
			vm_counterparty: self.vm_counterparty,
			events: self.events,
			events_counterparty: self.events_counterparty,
			admin: self.admin,
			admin_counterparty: self.admin_counterparty,
			block: self.block,
			block_counterparty: self.block_counterparty,
			shared: Connected { network },
		})
	}
}

impl<T> CrossChainScenario<XCVMState<T>, Connected> {
	fn dispatch_and_relay(
		&mut self,
		relayer: Account,
		relayer_counterparty: Account,
		sender: Account,
		program: DefaultXCVMProgram,
		salt: impl Into<Salt>,
		assets: impl IntoIterator<Item = (AssetId, u128)>,
		allowance_expiration: Option<Expiration>,
	) -> Result<CrossChainDispatchResult, TestError> {
		let tx_sender = self.mk_tx(sender);
		let tx_relayer = self.mk_tx(relayer);
		let tx_relayer_counterparty = self.mk_tx_counterparty(relayer_counterparty);
		let (dispatch_data, dispatch_events) = self.vm.dispatch_program_with_allowance(
			tx_sender,
			salt.into(),
			program,
			assets,
			allowance_expiration,
		)?;
		let (relay_data, relay_events) = self.shared.network.relay(
			&mut self.vm,
			&mut self.vm_counterparty,
			tx_relayer,
			tx_relayer_counterparty,
			u64::MAX,
		)?;
		Ok(CrossChainDispatchResult { dispatch_data, dispatch_events, relay_data, relay_events })
	}
	fn dispatch_and_relay_counterparty(
		&mut self,
		relayer: Account,
		relayer_counterparty: Account,
		sender: Account,
		program: DefaultXCVMProgram,
		salt: impl Into<Salt>,
		assets: impl IntoIterator<Item = (AssetId, u128)>,
		allowance_expiration: Option<Expiration>,
	) -> Result<CrossChainDispatchResult, TestError> {
		let tx_sender = self.mk_tx_counterparty(sender);
		let tx_relayer = self.mk_tx_counterparty(relayer);
		let tx_relayer_counterparty = self.mk_tx(relayer_counterparty);
		let (dispatch_data, dispatch_events) =
			self.vm_counterparty.dispatch_program_with_allowance(
				tx_sender,
				salt.into(),
				program,
				assets,
				allowance_expiration,
			)?;
		let (relay_data, relay_events) = self.shared.network.relay(
			&mut self.vm_counterparty,
			&mut self.vm,
			tx_relayer_counterparty,
			tx_relayer,
			u64::MAX,
		)?;
		Ok(CrossChainDispatchResult { dispatch_data, dispatch_events, relay_data, relay_events })
	}
}

fn mk_tx(acc: Account) -> BlockchainTransaction {
	BlockchainTransaction {
		block: BlockInfo {
			height: 1000,
			time: Timestamp::from_seconds(0),
			chain_id: "PICASSO-MEMNET".into(),
		},
		transaction: None,
		info: MessageInfo { sender: acc.into(), funds: Default::default() },
		gas: 1_000_000_000,
	}
}

fn to_canonical(account: Account) -> CanonicalAddr {
	SubstrateAddressHandler::addr_canonicalize(&String::from(account))
		.expect("impossible")
		.into()
}

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

fn create_base_xcvm_vm<N: Network, T>(
	tx: BlockchainTransaction,
) -> (TestVM<XCVMState<T>>, XCVMDeploymentEvents) {
	create_vm::<N>()
		.deploy_xcvm::<T>(tx)
		.expect("Must be able to deploy XCVM contracts.")
}

fn create_ready_xcvm_network<N: Network, M: Network, T>(
	block: BlockInfo,
	block_counterparty: BlockInfo,
	admin: Account,
	admin_counterparty: Account,
	relayer: Account,
	relayer_counterparty: Account,
	channel_id: impl Into<String>,
	connection_id: impl Into<String>,
	ordering: IbcOrder,
	pica_balances: impl IntoIterator<Item = Cw20Coin>,
	eth_balances: impl IntoIterator<Item = Cw20Coin>,
	usdt_balances: impl IntoIterator<Item = Cw20Coin>,
	usdc_balances: impl IntoIterator<Item = Cw20Coin>,
	pica_balances_counterparty: impl IntoIterator<Item = Cw20Coin>,
	eth_balances_counterparty: impl IntoIterator<Item = Cw20Coin>,
	usdt_balances_counterparty: impl IntoIterator<Item = Cw20Coin>,
	usdc_balances_counterparty: impl IntoIterator<Item = Cw20Coin>,
) -> Result<CrossChainScenario<XCVMState<T>, Connected>, TestError> {
	let mut network = CrossChainScenario::<XCVMState<T>, _>::new::<N, M>(
		block,
		block_counterparty,
		admin,
		admin_counterparty,
	);
	network.deploy_asset::<PICA>(pica_balances);
	network.deploy_asset::<ETH>(eth_balances);
	network.deploy_asset::<USDT>(usdt_balances);
	network.deploy_asset::<USDC>(usdc_balances);
	network.deploy_asset_counterparty::<PICA>(pica_balances_counterparty);
	network.deploy_asset_counterparty::<ETH>(eth_balances_counterparty);
	network.deploy_asset_counterparty::<USDT>(usdt_balances_counterparty);
	network.deploy_asset_counterparty::<USDC>(usdc_balances_counterparty);
	let network =
		network.connect(channel_id, connection_id, ordering, relayer, relayer_counterparty)?;
	Ok(network)
}

fn find_events<'a>(
	events: impl Iterator<Item = &'a Event>,
	ty: impl Into<String>,
) -> impl Iterator<Item = &'a Event> {
	let ty = ty.into();
	events.filter(move |x| x.ty == ty)
}

fn find_attr<'a>(
	mut attrs: impl Iterator<Item = &'a Attribute>,
	key: impl Into<String>,
) -> Option<&'a Attribute> {
	let key = key.into();
	attrs.find(|x| x.key == key)
}

fn xcvm_assert_prefixed_event<'a>(
	events: impl Iterator<Item = &'a Event> + Clone,
	ty: &str,
	key: &str,
	value: &str,
) {
	assert_event(events, &format!("{CUSTOM_CONTRACT_EVENT_PREFIX}{ty}"), key, value);
}

fn assert_event<'a>(
	events: impl Iterator<Item = &'a Event> + Clone,
	ty: &str,
	key: &str,
	value: &str,
) {
	let attr = find_events(events.clone(), format!("{ty}"))
		.find(|e| find_attr(e.attributes.iter(), key).filter(|a| a.value == value).is_some());
	assert_matches!(
		attr,
		Some(_),
		"Expected an event type: [{}] to contain the attribute key: [{}] and value: [{}] but got {:?}",
		ty,
		key,
		value,
		events.collect::<Vec<_>>()
	);
}

#[test]
fn test_deploy() {
	let admin = Account::unchecked("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
	let tx = mk_tx(admin);
	let (_, events) = create_vm::<Picasso>()
		.deploy_xcvm::<()>(tx)
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
	vm: &mut TestVM<XCVMState<T>>,
	tx: BlockchainTransaction,
	initial_balances: impl IntoIterator<Item = Cw20Coin>,
) {
	let symbol = A::SYMBOL;
	let gateway = vm.xcvm_state.gateway.clone();
	let (asset_address, events) = vm
		.deploy_asset::<A>(
			tx,
			initial_balances,
			Some(MinterResponse { minter: gateway.into(), cap: None }),
		)
		.expect(&format!("Must be able to instantiate and register {symbol} asset"));
	assert_eq!(events.registry_data, None);
	xcvm_assert_prefixed_event(
		events.registry_events.iter(),
		XCVM_ASSET_REGISTRY_EVENT_PREFIX,
		"action",
		"register",
	);
	xcvm_assert_prefixed_event(
		events.registry_events.iter(),
		XCVM_ASSET_REGISTRY_EVENT_PREFIX,
		"asset_id",
		&format!("{}", A::ID.0 .0),
	);
	xcvm_assert_prefixed_event(
		events.registry_events.iter(),
		XCVM_ASSET_REGISTRY_EVENT_PREFIX,
		"denom",
		&format!("cw20:{}", asset_address),
	);
}

#[test]
fn test_deploy_and_register_assets() {
	let admin = Account::unchecked("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
	let tx = mk_tx(admin);
	let (mut vm, _) = create_base_xcvm_vm::<Picasso, ()>(tx.clone());
	xcvm_deploy_asset::<PICA, _>(&mut vm, tx.clone(), []);
	xcvm_deploy_asset::<ETH, _>(&mut vm, tx.clone(), []);
	xcvm_deploy_asset::<USDT, _>(&mut vm, tx.clone(), []);
	xcvm_deploy_asset::<USDC, _>(&mut vm, tx, []);
}

fn simple_singlechain_xcvm_transfer(transfer_amount: u128) {
	let admin = Account::unchecked("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
	let admin_counterparty = Account::unchecked("5ES7Ks2P3i7ZadGdmCQq8LYyHrXLZKYXT66d6M6uU7h1Wmas");
	let relayer = Account::unchecked("5CSmbF3AUmspWJvjCm9LCJVwuQ1tZ2h4K5jwKPW51TXWb7H3");
	let relayer_counterparty =
		Account::unchecked("5CSR38vJcmiTc53kZrv36SFfxXd17SHgH6amp2H5HNgcBen7");
	let alice = Account::unchecked("5CSbGnoocQgFVJ9JfdeN5VArEB1XjqPfYxoSgEWuUjznuxTK");
	let bob = Account::unchecked("5ELSofPZUw6u2JUWsi7dQRkmR8HkT1LUFQvkTQdrcjEi3nbS");
	let block = BlockInfo {
		height: 1_000_000,
		time: Timestamp::from_seconds(1_000_000),
		chain_id: "PICASSO-MEMNET".into(),
	};
	let block_counterparty = BlockInfo {
		height: 12_000_000,
		time: Timestamp::from_seconds(1_000_000),
		chain_id: "JUNO-MEMNET".into(),
	};
	let mut network = create_ready_xcvm_network::<Picasso, Juno, ()>(
		block,
		block_counterparty,
		admin,
		admin_counterparty,
		relayer.clone(),
		relayer_counterparty.clone(),
		"ibc:in-memory",
		"ibc:connection:0",
		IbcOrder::Unordered,
		[Cw20Coin { address: alice.clone().into(), amount: transfer_amount.into() }],
		[],
		[],
		[],
		[],
		[],
		[],
		[],
	)
	.expect("Must be able to create an XCVM network.");
	let assets_to_transfer = [(PICA::ID, transfer_amount)];
	let program = ProgramBuilder::<Picasso, CanonicalAddr, Funds>::new([])
		.transfer(Destination::Account(to_canonical(bob.clone())), assets_to_transfer)
		.build();
	let CrossChainDispatchResult { dispatch_data, dispatch_events, relay_data, relay_events } =
		network
			.dispatch_and_relay(
				relayer,
				relayer_counterparty,
				alice.clone(),
				program,
				[],
				assets_to_transfer,
				None,
			)
			.expect("Must be able to transfer assets via XCVM");
  // We don't do cross-chain operation, nothing must happen from the relayer POV.
  assert_eq!(relay_data, Vec::default());
  assert_eq!(relay_events, Vec::default());

  // We don't dispatch any information in the data field.
	assert_eq!(dispatch_data, None);

	xcvm_assert_prefixed_event(
		dispatch_events.iter(),
		XCVM_ROUTER_EVENT_PREFIX,
		"action",
		"route.create",
	);
	xcvm_assert_prefixed_event(
		dispatch_events.iter(),
		XCVM_ROUTER_EVENT_PREFIX,
		"action",
		"route.execute",
	);
	xcvm_assert_prefixed_event(
		dispatch_events.iter(),
		XCVM_INTERPRETER_EVENT_PREFIX,
		"action",
		"execution.start",
	);
	xcvm_assert_prefixed_event(
		dispatch_events.iter(),
		XCVM_INTERPRETER_EVENT_PREFIX,
		"action",
		"execution.success",
	);
	assert_eq!(
		network.vm.balance_of::<PICA>(mk_tx(alice.clone()), alice.clone()),
		Ok(cw20::BalanceResponse { balance: 0_u128.into() })
	);
	assert_eq!(
		network.vm.balance_of::<PICA>(mk_tx(bob.clone()), bob.clone()),
		Ok(cw20::BalanceResponse { balance: transfer_amount.into() })
	);
}

// fn xcvm_crosschain_operation(
// 	vm: &mut TestVM<XCVMState<()>>,
// 	vm_counterparty: &mut TestVM<XCVMState<()>>,
// 	admin: Account,
// 	admin_counterparty: Account,
// 	relayer: Account,
// 	relayer_counterparty: Account,
// 	alice: Account,
// 	bob: Account,
// 	program: DefaultXCVMProgram,
// 	assets_to_transfer: &[(AssetId, u128)],
// 	allowance_expiration: Expiration,
// ) -> Result<(), TestError> {
// 	let channel_id = "ibc:in-memory:picasso-juno";
// 	let connection_id = "ibc:connection:0";
// 	let version = XCVM_GATEWAY_IBC_VERSION;
// 	let mut ibc_network = InMemoryIbcNetworkChannel::connect(
// 		vm,
// 		vm_counterparty,
// 		channel_id,
// 		connection_id,
// 		version,
// 		IbcOrder::Unordered,
// 		mk_tx(relayer.clone()),
// 		mk_tx(relayer_counterparty.clone()),
// 		mk_tx(admin),
// 		mk_tx(admin_counterparty),
// 		u64::MAX,
// 	)?;
// 	let (data, events) = vm.dispatch_program_with_allowance(
// 		mk_tx(alice.clone()),
// 		[],
// 		program,
// 		assets_to_transfer.iter().copied().collect::<Vec<_>>(),
// 		None,
// 	)?;
// 	ibc_network.relay(
// 		vm,
// 		vm_counterparty,
// 		mk_tx(relayer.clone()),
// 		mk_tx(relayer_counterparty.clone()),
// 		u64::MAX,
// 	)?;
// 	assert_eq!(data, None);
// 	xcvm_assert_prefixed_event(events.iter(), XCVM_ROUTER_EVENT_PREFIX, "action", "route.create");
// 	xcvm_assert_prefixed_event(events.iter(), XCVM_ROUTER_EVENT_PREFIX, "action", "route.execute");
// 	xcvm_assert_prefixed_event(
// 		events.iter(),
// 		XCVM_INTERPRETER_EVENT_PREFIX,
// 		"action",
// 		"execution.start",
// 	);
// 	xcvm_assert_prefixed_event(
// 		events.iter(),
// 		XCVM_INTERPRETER_EVENT_PREFIX,
// 		"action",
// 		"execution.success",
// 	);
// 	Ok(())
// }

// fn xcvm_crosschain_transfer(
// 	vm: &mut TestVM<XCVMState<()>>,
// 	vm_counterparty: &mut TestVM<XCVMState<()>>,
// 	admin: Account,
// 	admin_counterparty: Account,
// 	relayer: Account,
// 	relayer_counterparty: Account,
// 	alice: Account,
// 	bob: Account,
// 	transfer_amount: u128,
// ) -> Result<(), TestError> {
// 	assert_eq!(
// 		vm.balance_of::<PICA>(mk_tx(alice.clone()), alice.clone()),
// 		Ok(cw20::BalanceResponse { balance: transfer_amount.into() })
// 	);
// 	assert_eq!(
// 		vm_counterparty.balance_of::<PICA>(mk_tx(bob.clone()), bob.clone()),
// 		Ok(cw20::BalanceResponse { balance: 0_u128.into() })
// 	);
// 	assert_eq!(
// 		vm.token_info::<PICA>(mk_tx(alice.clone()))
// 			.expect("Must be able to query for asset info.")
// 			.total_supply
// 			.u128(),
// 		transfer_amount
// 	);
// 	assert_eq!(
// 		vm_counterparty
// 			.token_info::<PICA>(mk_tx(alice.clone()))
// 			.expect("Must be able to query for asset info.")
// 			.total_supply
// 			.u128(),
// 		0_u128
// 	);
// 	let channel_id = "ibc:in-memory:picasso-juno";
// 	let connection_id = "ibc:connection:0";
// 	let version = XCVM_GATEWAY_IBC_VERSION;
// 	let mut ibc_network = InMemoryIbcNetworkChannel::connect(
// 		vm,
// 		vm_counterparty,
// 		channel_id,
// 		connection_id,
// 		version,
// 		IbcOrder::Unordered,
// 		mk_tx(relayer.clone()),
// 		mk_tx(relayer_counterparty.clone()),
// 		mk_tx(admin),
// 		mk_tx(admin_counterparty),
// 		u64::MAX,
// 	)?;
// 	let assets_to_transfer = [(PICA::ID, transfer_amount)];
// 	let program = ProgramBuilder::<Picasso, CanonicalAddr, Funds>::new([])
// 		.spawn::<Juno, (), _, _>(
// 			"PHONE",
// 			[],
// 			BridgeSecurity::Deterministic,
// 			[(PICA::ID, Amount::everything())],
// 			|juno_program| {
// 				juno_program
// 					.transfer(
// 						Destination::Account(to_canonical(bob.clone())),
// 						[(PICA::ID, Amount::ratio(Amount::MAX_PARTS / 2))],
// 					)
// 					.spawn::<Picasso, (), _, _>(
// 						"HOME",
// 						[],
// 						BridgeSecurity::Deterministic,
// 						[(PICA::ID, Amount::everything())],
// 						|picasso_program| {
// 							Ok(picasso_program
// 								.transfer(Destination::Relayer, [(PICA::ID, Amount::everything())]))
// 						},
// 					)
// 			},
// 		)
// 		.expect("Must be able to create XCVM program.")
// 		.build();
// 	let (data, events) = vm.dispatch_program_with_allowance(
// 		mk_tx(alice.clone()),
// 		[],
// 		program,
// 		assets_to_transfer,
// 		None,
// 	)?;
// 	ibc_network.relay(
// 		vm,
// 		vm_counterparty,
// 		mk_tx(relayer.clone()),
// 		mk_tx(relayer_counterparty.clone()),
// 		u64::MAX,
// 	)?;
// 	assert_eq!(data, None);
// 	xcvm_assert_prefixed_event(events.iter(), XCVM_ROUTER_EVENT_PREFIX, "action", "route.create");
// 	xcvm_assert_prefixed_event(events.iter(), XCVM_ROUTER_EVENT_PREFIX, "action", "route.execute");
// 	xcvm_assert_prefixed_event(
// 		events.iter(),
// 		XCVM_INTERPRETER_EVENT_PREFIX,
// 		"action",
// 		"execution.start",
// 	);
// 	xcvm_assert_prefixed_event(
// 		events.iter(),
// 		XCVM_INTERPRETER_EVENT_PREFIX,
// 		"action",
// 		"execution.success",
// 	);
// 	assert_eq!(
// 		vm.balance_of::<PICA>(mk_tx(alice.clone()), alice.clone()),
// 		Ok(cw20::BalanceResponse { balance: 0_u128.into() })
// 	);
// 	assert_ok!(almost_eq(
// 		vm_counterparty
// 			.balance_of::<PICA>(mk_tx(bob.clone()), bob.clone())
// 			.unwrap()
// 			.balance
// 			.u128(),
// 		transfer_amount / 2,
// 		1
// 	));
// 	assert_ok!(almost_eq(
// 		vm.balance_of::<PICA>(mk_tx(relayer.clone()), relayer_counterparty.clone())
// 			.unwrap()
// 			.balance
// 			.u128(),
// 		transfer_amount / 2,
// 		1
// 	));
// 	let supply = vm
// 		.token_info::<PICA>(mk_tx(alice.clone()))
// 		.expect("Must be able to query for asset info.")
// 		.total_supply
// 		.u128();
// 	let supply_counterparty = vm_counterparty
// 		.token_info::<PICA>(mk_tx(alice.clone()))
// 		.expect("Must be able to query for asset info.")
// 		.total_supply
// 		.u128();
// 	assert_eq!(supply + supply_counterparty, transfer_amount);
// 	Ok(())
// }

// fn xcvm_crosschain_transfer_ok(transfer_amount: u128) {
// 	let admin_picasso = Account::unchecked("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
// 	let admin_juno = Account::unchecked("5CS3GHP2SKycDJWpEziHrqYQU8L62kVFmM5CDKYDipRv9XAc");
// 	let relayer_picasso = Account::unchecked("5CSmbF3AUmspWJvjCm9LCJVwuQ1tZ2h4K5jwKPW51TXWb7H3");
// 	let relayer_juno = Account::unchecked("5CSR38vJcmiTc53kZrv36SFfxXd17SHgH6amp2H5HNgcBen7");
// 	let alice = Account::unchecked("5CSbGnoocQgFVJ9JfdeN5VArEB1XjqPfYxoSgEWuUjznuxTK");
// 	let bob = Account::unchecked("5ELSofPZUw6u2JUWsi7dQRkmR8HkT1LUFQvkTQdrcjEi3nbS");
// 	let mut vm_picasso = create_xcvm_vm::<Picasso, ()>(
// 		mk_tx(admin_picasso.clone()),
// 		[Cw20Coin { address: alice.clone().into(), amount: transfer_amount.into() }],
// 		[],
// 		[],
// 		[],
// 	);
// 	let mut vm_juno = create_xcvm_vm::<Juno, ()>(mk_tx(admin_juno.clone()), [], [], [], []);
// 	assert_ok!(xcvm_crosschain_transfer(
// 		&mut vm_picasso,
// 		&mut vm_juno,
// 		admin_picasso,
// 		admin_juno,
// 		relayer_picasso,
// 		relayer_juno,
// 		alice,
// 		bob,
// 		transfer_amount,
// 	));
// }

mod property {
	use super::*;
	proptest! {
	  #[test]
	  fn test_simple_singlechain_xcvm_transfer(transfer_amount in 0u128..u128::MAX) {
		  simple_singlechain_xcvm_transfer(transfer_amount);
	  }
	  // #[test]
	  // fn test_xcvm_crosschain_transfer_ok(transfer_amount in 0u128..u128::MAX) {
		//   xcvm_crosschain_transfer_ok(transfer_amount)
	  // }
	}
}
