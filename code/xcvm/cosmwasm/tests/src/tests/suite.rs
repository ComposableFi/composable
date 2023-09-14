use super::framework::{
	InMemoryIbcNetworkChannel, TestError, XCVMContracts, XCVMDeploymentEvents, XCVMState,
};
use crate::tests::framework::{BlockchainTransaction, TestVM};
use cosmwasm_orchestrate::vm::{Account, AddressHandler, SubstrateAddressHandler};
use cosmwasm_std::{Attribute, Binary, BlockInfo, Event, IbcOrder, MessageInfo, Timestamp};
use cosmwasm_vm::system::CUSTOM_CONTRACT_EVENT_PREFIX;
use cw20::{Cw20Coin, Expiration, MinterResponse};

use proptest::{prelude::any, prop_assume, prop_compose, proptest};
use std::assert_matches::assert_matches;
use xc_core::{
	shared, AssetId, Balance, Centauri, Destination, Funds, Network, Picasso, ProgramBuilder,
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

prop_compose! {
	fn account()
		(bytes in any::<[u8; 32]>()) -> Account {
			Account::unchecked(SubstrateAddressHandler::addr_generate([&bytes[..]]).expect("impossible; qed;"))
		}
}

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
	fn deploy_asset(
		&mut self,
		initial_balances: impl IntoIterator<Item = Cw20Coin>,
		asset_id: AssetId,
	) {
		let tx = self.mk_tx(self.admin.clone());
		xcvm_deploy_asset::<T>(&mut self.vm, tx, initial_balances, asset_id)
	}

	fn deploy_asset_counterparty(
		&mut self,
		initial_balances: impl IntoIterator<Item = Cw20Coin>,
		asset_id: AssetId,
	) {
		let tx = self.mk_tx_counterparty(self.admin_counterparty.clone());
		xcvm_deploy_asset::<T>(&mut self.vm_counterparty, tx, initial_balances, asset_id)
	}
}

impl<T> CrossChainScenario<T, Disconnected> {
	fn new<M: Network, N: Network>(
		block: BlockInfo,
		block_counterparty: BlockInfo,
		admin: Account,
		admin_counterparty: Account,
	) -> Self {
		let (vm, events) = create_base_xcvm_vm::<M, T>(mk_tx_raw(block.clone(), admin.clone()));
		let (vm_counterparty, events_counterparty) = create_base_xcvm_vm::<N, T>(mk_tx_raw(
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
		const VERSION: &str = xc_core::gateway::IBC_VERSION;
		let tx_relayer = self.mk_tx(relayer);
		let tx_relayer_counterparty = self.mk_tx_counterparty(relayer_counterparty);
		let tx_admin = self.mk_tx(self.admin.clone());
		let tx_admin_counterparty = self.mk_tx_counterparty(self.admin_counterparty.clone());
		let network = InMemoryIbcNetworkChannel::connect(
			&mut self.vm,
			&mut self.vm_counterparty,
			channel_id.into(),
			connection_id.into(),
			VERSION,
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
		program: shared::XcProgram,
		salt: impl Into<shared::Salt>,
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
		program: shared::XcProgram,
		salt: impl Into<shared::Salt>,
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

fn to_xc_addr(account: Account) -> shared::XcAddr {
	SubstrateAddressHandler::addr_canonicalize(&String::from(account))
		.expect("impossible")
		.into()
}

/// Loads compiled WASM contracts used in tests.
///
/// Firstly, loads CosmWasm contracts implementing XCVM.  Those are read from
/// `target/wasm32-unknown-unknown/cosmwasm-contracts` directory.  The
/// expectation is that **the user will compile the contracts** prior to running
/// the tests.  This can be done by executing `build-contracts.sh` script.
///
/// Alternatively, location of those contract files can be specified via
/// corresponding `CW_XCVM_xxx` environment variables (see function body for
/// exact variable names and which contracts the map to).
///
/// **Note**: The downside of this approach is that the contracts aren’t
/// automatically rebuilt if any of the contracts (or their dependencies) is
/// edited.  It is user’s responsibility to do it.
///
/// Secondly, loads `cw20_base.wasm` contract from `$OUT_DIR`.  The expectation
/// is that Cargo build script downloaded that contract and puts it in the
/// output directory.  However, when running on CI, the build script won’t
/// download the file and instead its location must be specified via `CW20`
/// environment variable.
fn load_contracts() -> XCVMContracts {
	fn read(path: impl std::convert::AsRef<std::path::Path>) -> Vec<u8> {
		fn imp(path: &std::path::Path) -> Vec<u8> {
			std::fs::read(path).unwrap_or_else(|err| panic!("{}: {err}", path.display()))
		}
		imp(path.as_ref())
	}

	// TODO(mina86): Figure out a better way of handling this where contracts
	// are automatically rebuilt when they are changed.  Build script doesn’t
	// solve the issue since `cargo:rerun-if-changed` mechanism is insufficient
	// to catch dependencies in the contract.  Perhaps it makes sense to always
	// build the contracts?  Presumably if they were not changed, `cargo build`
	// will be quick.
	fn read_contract(filename: &str) -> Vec<u8> {
		let contracts_dir = std::path::Path::new(concat!(
			env!("CARGO_MANIFEST_DIR"),
			"/../../target/wasm32-unknown-unknown/cosmwasm-contracts",
		));
		read(contracts_dir.join(filename))
	}

	let code_interpreter = read_contract("cw_xc_interpreter.wasm");
	let code_gateway = read_contract("cw_xc_gateway.wasm");
	let out_dir =
		[option_env!("NIX_CARGO_OUT_DIR").unwrap_or(env!("OUT_DIR")), "/cw20_base.wasm"].concat();
	// When running the test locally, the cw20_base.wasm file is downloaded by
	// the Build script (see build.rs).
	let code_cw20 = read(&out_dir);

	XCVMContracts::new(code_interpreter, code_gateway, code_cw20)
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

fn create_ready_xcvm_network<M: Network, N: Network, T>(
	block: BlockInfo,
	block_counterparty: BlockInfo,
	admin: Account,
	admin_counterparty: Account,
	relayer: Account,
	relayer_counterparty: Account,
	pica_balances: impl IntoIterator<Item = Cw20Coin>,
	eth_balances: impl IntoIterator<Item = Cw20Coin>,
	usdt_balances: impl IntoIterator<Item = Cw20Coin>,
	usdc_balances: impl IntoIterator<Item = Cw20Coin>,
	pica_balances_counterparty: impl IntoIterator<Item = Cw20Coin>,
	eth_balances_counterparty: impl IntoIterator<Item = Cw20Coin>,
	usdt_balances_counterparty: impl IntoIterator<Item = Cw20Coin>,
	usdc_balances_counterparty: impl IntoIterator<Item = Cw20Coin>,
	channel_id: impl Into<String>,
	connection_id: impl Into<String>,
	ordering: IbcOrder,
) -> Result<CrossChainScenario<XCVMState<T>, Connected>, TestError> {
	let mut network = CrossChainScenario::<XCVMState<T>, _>::new::<M, N>(
		block,
		block_counterparty,
		admin,
		admin_counterparty,
	);
	network.deploy_asset(pica_balances, 1.into());
	network.deploy_asset(eth_balances, 2.into());
	network.deploy_asset(usdt_balances, 3.into());
	network.deploy_asset(usdc_balances, 4.into());
	network.deploy_asset_counterparty(pica_balances_counterparty, 5.into());
	network.deploy_asset_counterparty(eth_balances_counterparty, 6.into());
	network.deploy_asset_counterparty(usdt_balances_counterparty, 7.into());
	network.deploy_asset_counterparty(usdc_balances_counterparty, 8.into());
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

#[track_caller]
fn assert_event<'a>(
	events: impl Iterator<Item = &'a Event> + Clone,
	ty: &str,
	key: &str,
	value: &str,
) {
	let attr = find_events(events.clone(), ty)
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

fn xcvm_deploy_asset<T>(
	vm: &mut TestVM<XCVMState<T>>,
	tx: BlockchainTransaction,
	initial_balances: impl IntoIterator<Item = Cw20Coin>,
	asset_id: AssetId,
) {
	let symbol = "DNC";
	let gateway = vm.xcvm_state.gateway.clone();
	let (asset_address, events) = vm
		.deploy_asset(
			tx,
			initial_balances,
			Some(MinterResponse { minter: gateway.into(), cap: None }),
			asset_id,
		)
		.expect(&format!("Must be able to instantiate and register {symbol} asset"));
	assert_eq!(events.gateway_data, None);
}

mod base {
	use super::*;
	use crate::tests::framework::XCVMRegisterAssetEvents;
	use cosmwasm_orchestrate::vm::VmError;
	use cosmwasm_vm::system::SystemError;
	use cosmwasm_vm_wasmi::WasmiVMError;

	fn deploy(admin: Account) {
		let tx = mk_tx(admin);
		let (_, events) = create_vm::<Picasso>()
			.deploy_xcvm::<()>(tx)
			.expect("Must be able to deploy XCVM contracts.");
		assert_eq!(events.gateway_data, None);
	}

	fn deploy_and_register_assets(
		asset_id: AssetId,
		admin: Account,
		arbitrary_sender: Account,
	) -> Result<(Account, XCVMRegisterAssetEvents), TestError> {
		let tx = mk_tx(admin);
		let tx_arbitrary = mk_tx(arbitrary_sender);
		let (mut vm, _) = create_base_xcvm_vm::<Picasso, ()>(tx.clone());
		let gateway = vm.xcvm_state.gateway.clone();
		vm.deploy_asset(
			tx_arbitrary,
			[],
			Some(MinterResponse { minter: gateway.into(), cap: None }),
			asset_id,
		)
	}

	fn arbitrary_user_cannot_register_asset(admin: Account, arbitrary_sender: Account) {
		assert_eq!(
			deploy_and_register_assets(1u128.into(), admin, arbitrary_sender),
			Err(TestError::Vm(VmError::VMError(WasmiVMError::SystemError(
				SystemError::ContractExecutionFailure(
					"Caller is not authorised to take this action.".into()
				)
			))))
		);
	}

	fn admin_can_register_asset(admin: Account) {
		assert_ok!(deploy_and_register_assets(1u128.into(), admin.clone(), admin.clone()));
	}

	proptest! {
		#[test]
		fn test_deploy(admin in account()) {
		  deploy(admin)
		}

		#[test]
		fn test_arbitrary_user_cannot_register_asset(admin in account(), arbitrary_sender in account()) {
		  prop_assume!(admin != arbitrary_sender);
		  arbitrary_user_cannot_register_asset(admin, arbitrary_sender);
		}

		#[test]
		fn test_admin_can_register_asset(admin in account()) {
		  admin_can_register_asset(admin)
		}
	}
}

mod single_chain {
	use xc_core::Centauri;

	use super::*;

	fn simple_singlechain_xcvm_transfer(
		admin: Account,
		admin_counterparty: Account,
		relayer: Account,
		relayer_counterparty: Account,
		alice: Account,
		bob: Account,
		transfer_amount: u128,
	) {
		let block = BlockInfo {
			height: 1_000_000,
			time: Timestamp::from_seconds(1_000_000),
			chain_id: "PICASSO-MEMNET".into(),
		};
		let block_counterparty = BlockInfo {
			height: 12_000_000,
			time: Timestamp::from_seconds(1_000_000),
			chain_id: "Centauri-MEMNET".into(),
		};
		let mut network = create_ready_xcvm_network::<Picasso, Centauri, ()>(
			block,
			block_counterparty,
			admin,
			admin_counterparty,
			relayer.clone(),
			relayer_counterparty.clone(),
			[Cw20Coin { address: alice.clone().into(), amount: transfer_amount.into() }],
			[],
			[],
			[],
			[],
			[],
			[],
			[],
			"ibc:in-memory",
			"ibc:connection:0",
			IbcOrder::Unordered,
		)
		.expect("Must be able to create an XCVM network.");
		let assets_to_transfer = [(1u128.into(), transfer_amount)];
		let program = ProgramBuilder::<Picasso, shared::XcAddr, Funds<Balance>>::new([])
			.transfer(Destination::Account(to_xc_addr(bob.clone())), assets_to_transfer)
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
		assert!(relay_data.is_empty());
		assert!(relay_events.is_empty());

		// We don't dispatch any information in the data field.
		assert_eq!(dispatch_data, None);

		assert_eq!(
			network.vm.balance_of(1u128.into(), mk_tx(alice.clone()), alice.clone()),
			Ok(cw20::BalanceResponse { balance: 0_u128.into() })
		);
		assert_eq!(
			network.vm.balance_of(1u128.into(), mk_tx(bob.clone()), bob.clone()),
			Ok(cw20::BalanceResponse { balance: transfer_amount.into() })
		);
	}

	proptest! {
	  #[test]
	  fn test_simple_singlechain_xcvm_transfer(
		  admin in account(),
		  admin_counterparty in account(),
		  relayer in account(),
		  relayer_counterparty in account(),
		  alice in account(),
		  bob in account(),
		  transfer_amount in 0u128..1024u128) {
		  simple_singlechain_xcvm_transfer(admin, admin_counterparty, relayer, relayer_counterparty, alice, bob, transfer_amount);
	  }
	}
}

mod cross_chain {
	use cosmwasm_std::Uint128;
	use xc_core::XCVMAck;

	use super::*;

	fn simple_crosschain_xcvm_transfer(
		admin: Account,
		admin_counterparty: Account,
		relayer: Account,
		relayer_counterparty: Account,
		alice: Account,
		bob: Account,
		transfer_amount: u128,
	) {
		let block = BlockInfo {
			height: 1_000_000,
			time: Timestamp::from_seconds(1_000_000),
			chain_id: "PICASSO-MEMNET".into(),
		};
		let block_counterparty = BlockInfo {
			height: 12_000_000,
			time: Timestamp::from_seconds(1_000_000),
			chain_id: "Centauri-MEMNET".into(),
		};
		let mut network = create_ready_xcvm_network::<Picasso, Centauri, ()>(
			block,
			block_counterparty,
			admin,
			admin_counterparty,
			relayer.clone(),
			relayer_counterparty.clone(),
			[Cw20Coin { address: alice.clone().into(), amount: transfer_amount.into() }],
			[],
			[],
			[],
			[],
			[],
			[],
			[],
			"ibc:in-memory",
			"ibc:connection:0",
			IbcOrder::Unordered,
		)
		.expect("Must be able to create an XCVM network.");
		let assets_to_transfer = [(1u128.into(), transfer_amount)];
		let program = ProgramBuilder::<Picasso, shared::XcAddr, Funds<Balance>>::new([])
			.spawn::<Centauri, (), _, _>([], [], assets_to_transfer, |Centauri_program| {
				Ok(Centauri_program
					.transfer(Destination::Account(to_xc_addr(bob.clone())), assets_to_transfer))
			})
			.expect("Must be able to build an XCVM program.")
			.build();
		let CrossChainDispatchResult { dispatch_data, relay_data, .. } = network
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

		// Source chain, both alice and bob have 0 tokens.
		assert_eq!(
			network.vm.balance_of(1u128.into(), network.mk_tx(alice.clone()), alice.clone()),
			Ok(cw20::BalanceResponse { balance: 0u128.into() })
		);
		assert_eq!(
			network.vm.balance_of(1u128.into(), network.mk_tx(bob.clone()), bob.clone()),
			Ok(cw20::BalanceResponse { balance: 0u128.into() })
		);

		// Destination, alice has 0 tokens and bob has the transferred amount.
		assert_eq!(
			network.vm_counterparty.balance_of(
				1u128.into(),
				network.mk_tx_counterparty(alice.clone()),
				alice.clone()
			),
			Ok(cw20::BalanceResponse { balance: 0u128.into() })
		);
		assert_eq!(
			network.vm_counterparty.balance_of(
				1u128.into(),
				network.mk_tx_counterparty(bob.clone()),
				bob.clone()
			),
			Ok(cw20::BalanceResponse { balance: transfer_amount.into() })
		);

		// The supply moved from the source chain to the destination chain.
		assert_matches!(network.vm.token_info(1u128.into(), network.mk_tx(alice.clone())), Ok(cw20::TokenInfoResponse {
      total_supply,
      ..
    }) if total_supply == Uint128::zero());
		assert_matches!(network.vm_counterparty.token_info(1u128.into(), network.mk_tx_counterparty(alice.clone())), Ok(cw20::TokenInfoResponse {
      total_supply,
      ..
    }) if total_supply == Uint128::from(transfer_amount));

		// The relayer must obtain a successful ack on the destination, and nothing on the source
		// after relaying the ack itself.
		assert_eq!(relay_data, vec![Some(XCVMAck::Ok.into()), None]);

		// We don't dispatch any information in the data field.
		assert_eq!(dispatch_data, None);
	}

	proptest! {
	  #[ignore] // until ICS-20 integraion for assets
	  #[test]
	  fn test_simple_crosschain_xcvm_transfer(
		  admin in account(),
		  admin_counterparty in account(),
		  relayer in account(),
		  relayer_counterparty in account(),
		  alice in account(),
		  bob in account(),
		  transfer_amount in 0u128..1024u128) {
		  simple_crosschain_xcvm_transfer(admin, admin_counterparty, relayer, relayer_counterparty, alice, bob, transfer_amount);
	  }
	}
}
