use cosmwasm_orchestrate::{
	ibc::IbcNetwork,
	vm::{Account, IbcChannelId, State, SubstrateAddressHandler, VmError},
	Dispatch, ExecutionType, StateBuilder, SubstrateApi,
};
use cosmwasm_std::{
	Binary, BlockInfo, ContractInfo, Env, Event, IbcChannel, IbcOrder, MessageInfo, TransactionInfo,
};
use cosmwasm_vm::system::CosmwasmCodeId;
use cw20::{Cw20Coin, MinterResponse};
use cw_xcvm_asset_registry::msg::AssetReference;
use cw_xcvm_router::contract::XCVM_ROUTER_EVENT_PREFIX;
use std::{collections::HashMap, hash::Hash};
use xcvm_core::{Asset, AssetSymbol, Network, NetworkId};

pub const XCVM_ASSET_REGISTRY_CODE: CosmwasmCodeId = 0;
pub const XCVM_INTERPRETER_CODE: CosmwasmCodeId = 1;
pub const XCVM_ROUTER_CODE: CosmwasmCodeId = 2;
pub const XCVM_GATEWAY_CODE: CosmwasmCodeId = 3;
pub const XCVM_CW20_CODE: CosmwasmCodeId = 4;
pub const RESERVED_CODE_LIMIT: CosmwasmCodeId = XCVM_CW20_CODE;

pub type TestDispatch = Dispatch;

pub type TestApi<'a> = SubstrateApi<'a, TestDispatch>;

pub type VMState = State<(), SubstrateAddressHandler>;

pub type InMemoryIbcNetwork<'a> = IbcNetwork<'a, (), SubstrateAddressHandler>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockchainTransaction {
	pub block: BlockInfo,
	pub transaction: Option<TransactionInfo>,
	pub info: MessageInfo,
	pub gas: u64,
}

#[derive(PartialEq, Eq, Debug)]
pub struct XCVMDeploymentEvents {
	pub registry_data: Option<Binary>,
	pub registry_events: Vec<Event>,
	pub gateway_data: Option<Binary>,
	pub gateway_events: Vec<Event>,
}

pub struct XCVMRegisterAssetEvents {
	pub asset_data: Option<Binary>,
	pub asset_events: Vec<Event>,
	pub registry_data: Option<Binary>,
	pub registry_events: Vec<Event>,
}

#[derive(Debug)]
pub enum TestError {
	Vm(VmError),
	ChannelCollision(IbcChannelId),
}

impl From<VmError> for TestError {
	fn from(value: VmError) -> Self {
		Self::Vm(value)
	}
}

pub struct XCVMContracts {
	asset_registry: Vec<u8>,
	interpreter: Vec<u8>,
	router: Vec<u8>,
	gateway: Vec<u8>,
	cw20: Vec<u8>,
}

impl XCVMContracts {
	pub fn new(
		asset_registry: Vec<u8>,
		interpreter: Vec<u8>,
		router: Vec<u8>,
		gateway: Vec<u8>,
		cw20: Vec<u8>,
	) -> Self {
		Self { asset_registry, interpreter, router, gateway, cw20 }
	}
}

pub struct InMemoryIbcNetworkChannel<'a, T> {
	vm: &'a mut T,
	vm_counterparty: &'a mut T,
	channel_id: IbcChannelId,
	channel: IbcChannel,
}

impl<'a> InMemoryIbcNetworkChannel<'a, TestVM<()>> {
	pub fn connect<T: Clone + Into<IbcChannelId>>(
		xcvm_contracts: XCVMContracts,
		vm: &'a mut TestVM<()>,
		vm_counterparty: &'a mut TestVM<()>,
		channel_id: T,
		connection_id: impl Into<String>,
		version: impl Into<String>,
		ordering: impl Into<IbcOrder>,
		env: impl Into<Env>,
		env_counterparty: impl Into<Env>,
		info: impl Into<MessageInfo>,
		info_counterparty: impl Into<MessageInfo>,
		gas: impl Into<u64>,
	) -> Result<InMemoryIbcNetworkChannel<'a, TestVM<()>>, TestError> {
		let channel = InMemoryIbcNetwork::new(&mut vm.vm_state, &mut vm_counterparty.vm_state)
			.handshake(
				channel_id.clone().into(),
				version.into(),
				ordering.into(),
				connection_id.into(),
				env.into(),
				env_counterparty.into(),
				info.into(),
				info_counterparty.into(),
				gas.into(),
			)?;
		Ok(Self { vm, vm_counterparty, channel_id: channel_id.into(), channel })
	}
}

pub struct TestVM<T> {
	network_id: NetworkId,
	vm_state: VMState,
	xcvm_state: T,
}

impl TestVM<()> {
	pub fn new<N: Network>(contracts: XCVMContracts) -> Self {
		Self::new_with_config::<N>(contracts, |config| config)
	}

	pub fn new_with_config<N: Network>(
		XCVMContracts { asset_registry, interpreter, router, gateway, cw20 }: XCVMContracts,
		config: impl FnOnce(
			StateBuilder<SubstrateAddressHandler>,
		) -> StateBuilder<SubstrateAddressHandler>,
	) -> Self {
		let vm_state = config(StateBuilder::<SubstrateAddressHandler>::new())
			.add_code(XCVM_ASSET_REGISTRY_CODE, asset_registry)
			.add_code(XCVM_INTERPRETER_CODE, interpreter)
			.add_code(XCVM_ROUTER_CODE, router)
			.add_code(XCVM_GATEWAY_CODE, gateway)
			.add_code(XCVM_CW20_CODE, cw20)
			.build();
		Self { network_id: N::ID, vm_state, xcvm_state: () }
	}

	pub fn deploy_xcvm<T>(
		mut self,
		tx: BlockchainTransaction,
	) -> Result<(TestVM<XCVMState<T>>, XCVMDeploymentEvents), TestError> {
		let (registry_address, (registry_data, registry_events)) = TestApi::instantiate(
			&mut self.vm_state,
			XCVM_ASSET_REGISTRY_CODE,
			None,
			tx.block.clone(),
			tx.transaction.clone(),
			tx.info.clone(),
			tx.gas,
			cw_xcvm_asset_registry::msg::InstantiateMsg {},
		)?;

		// The gateway deploy the router under the hood.
		let (gateway_address, (gateway_data, gateway_events)) =
			SubstrateApi::<Dispatch>::instantiate(
				&mut self.vm_state,
				XCVM_GATEWAY_CODE,
				None,
				tx.block,
				tx.transaction,
				tx.info.clone(),
				tx.gas,
				cw_xcvm_gateway::msg::InstantiateMsg {
					config: cw_xcvm_gateway::state::Config {
						registry_address: registry_address.clone().to_string(),
						router_code_id: XCVM_ROUTER_CODE,
						interpreter_code_id: XCVM_INTERPRETER_CODE,
						network_id: self.network_id,
						admin: tx.info.sender.into_string(),
					},
				},
			)?;

		let router_address: Account = gateway_events
			.iter()
			.find_map(|e| {
				if e.ty == format!("wasm-{}", XCVM_ROUTER_EVENT_PREFIX) {
					e.attributes.iter().find(|a| a.key == "_contract_address")
				} else {
					None
				}
			})
			.expect("The XCVM gateway must instantiate the XCVM router.")
			.value
			.clone()
			.try_into()
			.expect("The XCVM router addresss must be valid");

		Ok((
			TestVM {
				network_id: self.network_id,
				vm_state: self.vm_state,
				xcvm_state: XCVMState::new(registry_address, gateway_address, router_address),
			},
			XCVMDeploymentEvents { registry_data, registry_events, gateway_data, gateway_events },
		))
	}
}

impl<T> TestVM<XCVMState<T>> {
	pub fn deploy_asset<A: Asset + AssetSymbol>(
		mut self,
		tx: BlockchainTransaction,
		initial_balances: Vec<Cw20Coin>,
		mint: Option<MinterResponse>,
	) -> Result<(Self, XCVMRegisterAssetEvents), TestError> {
		let (asset_address, (asset_data, asset_events)) = TestApi::instantiate(
			&mut self.vm_state,
			XCVM_CW20_CODE,
			None,
			tx.block.clone(),
			tx.transaction.clone(),
			tx.info.clone(),
			tx.gas,
			cw20_base::msg::InstantiateMsg {
				name: A::SYMBOL.into(),
				symbol: A::SYMBOL.into(),
				decimals: A::DECIMALS,
				initial_balances,
				mint,
				marketing: None,
			},
		)?;

		let (registry_data, registry_events) = TestApi::execute(
			&mut self.vm_state,
			Env {
				block: tx.block.clone(),
				transaction: tx.transaction.clone(),
				contract: ContractInfo { address: self.xcvm_state.registry.clone().into() },
			},
			MessageInfo { sender: tx.info.sender, funds: Default::default() },
			tx.gas,
			cw_xcvm_asset_registry::msg::ExecuteMsg::RegisterAsset {
				asset_id: A::ID.into(),
				reference: AssetReference::Virtual { cw20_address: asset_address.clone().into() },
			},
		)?;

		Ok((
			self,
			XCVMRegisterAssetEvents { asset_data, asset_events, registry_data, registry_events },
		))
	}
}

pub struct XCVMState<T> {
	pub registry: Account,
	pub gateway: Account,
	pub router: Account,
	pub custom: HashMap<T, Account>,
}

impl<T> XCVMState<T> {
	pub fn new(registry: Account, gateway: Account, router: Account) -> Self {
		Self { registry, gateway, router, custom: Default::default() }
	}
}

impl<T> XCVMState<T>
where
	T: Eq + Hash,
{
	pub fn get_contract(&self, index: &T) -> Option<&Account> {
		self.custom.get(index)
	}
	pub fn insert_contract(&mut self, index: T, contract: Account) -> Option<Account> {
		self.custom.insert(index, contract)
	}
}
