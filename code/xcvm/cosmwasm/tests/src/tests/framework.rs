use cosmwasm_orchestrate::{
	ibc::{IbcHandshakeResult, IbcNetwork},
	vm::{Account, IbcChannelId, State, SubstrateAddressHandler, VmError},
	Direct, Dispatch, StateBuilder, SubstrateApi,
};
use cosmwasm_std::{
	Binary, BlockInfo, ContractInfo, Env, Event, IbcOrder, MessageInfo, TransactionInfo,
};
use cosmwasm_vm::system::CosmwasmCodeId;
use cw20::{Cw20Coin, Expiration, MinterResponse};
use std::{collections::HashMap, hash::Hash};
use xc_core::{
	gateway::{AssetItem, ConfigSubMsg, HereItem, NetworkItem},
	shared::{Salt, XcProgram},
	AssetId, Funds, Network, NetworkId,
};

pub const XCVM_INTERPRETER_CODE: CosmwasmCodeId = 0;
pub const XCVM_GATEWAY_CODE: CosmwasmCodeId = 1;
pub const XCVM_CW20_CODE: CosmwasmCodeId = 2;
pub const RESERVED_CODE_LIMIT: CosmwasmCodeId = XCVM_CW20_CODE;

pub type TestDispatch = Dispatch;

pub type TestQueryApi<'a> = SubstrateApi<'a, Direct>;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XCVMDeploymentEvents {
	pub gateway_data: Option<Binary>,
	pub gateway_events: Vec<Event>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XCVMRegisterAssetEvents {
	pub asset_data: Option<Binary>,
	pub asset_events: Vec<Event>,
	pub gateway_data: Option<Binary>,
	pub gateway_events: Vec<Event>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum TestError {
	Vm(VmError),
	AssetNotDeployed,
}

impl From<VmError> for TestError {
	fn from(value: VmError) -> Self {
		Self::Vm(value)
	}
}

pub struct XCVMContracts {
	interpreter: Vec<u8>,
	gateway: Vec<u8>,
	cw20: Vec<u8>,
}

impl XCVMContracts {
	pub fn new(interpreter: Vec<u8>, gateway: Vec<u8>, cw20: Vec<u8>) -> Self {
		Self { interpreter, gateway, cw20 }
	}
}

pub struct InMemoryIbcNetworkChannel {
	channel_id: IbcChannelId,
	handshake: IbcHandshakeResult,
}

impl InMemoryIbcNetworkChannel {
	pub fn connect<T, C: Clone + Into<IbcChannelId>>(
		vm: &mut TestVM<XCVMState<T>>,
		vm_counterparty: &mut TestVM<XCVMState<T>>,
		channel_id: C,
		connection_id: impl Into<String>,
		version: impl Into<String>,
		ordering: impl Into<IbcOrder>,
		tx_relayer: BlockchainTransaction,
		tx_relayer_counterparty: BlockchainTransaction,
		tx_admin: BlockchainTransaction,
		tx_admin_counterparty: BlockchainTransaction,
		gas: u64,
	) -> Result<InMemoryIbcNetworkChannel, TestError> {
		let channel_id = channel_id.into();
		let handshake = InMemoryIbcNetwork::new(&mut vm.vm_state, &mut vm_counterparty.vm_state)
			.handshake(
				channel_id.clone(),
				version.into(),
				ordering.into(),
				connection_id.into(),
				Env {
					block: tx_relayer.block.clone(),
					transaction: tx_relayer.transaction.clone(),
					contract: ContractInfo { address: vm.xcvm_state.gateway.clone().into() },
				},
				Env {
					block: tx_relayer_counterparty.block.clone(),
					transaction: tx_relayer_counterparty.transaction.clone(),
					contract: ContractInfo {
						address: vm_counterparty.xcvm_state.gateway.clone().into(),
					},
				},
				tx_relayer.info.clone(),
				tx_relayer_counterparty.info.clone(),
				gas,
			)?;
		TestApi::execute(
			&mut vm.vm_state,
			Env {
				block: tx_admin.block.clone(),
				transaction: tx_admin.transaction.clone(),
				contract: ContractInfo { address: vm.xcvm_state.gateway.clone().into() },
			},
			tx_admin.info.clone(),
			gas,
			todo!(),
		)?;
		TestApi::execute(
			&mut vm_counterparty.vm_state,
			Env {
				block: tx_admin_counterparty.block.clone(),
				transaction: tx_admin_counterparty.transaction.clone(),
				contract: ContractInfo {
					address: vm_counterparty.xcvm_state.gateway.clone().into(),
				},
			},
			tx_admin_counterparty.info.clone(),
			gas,
			todo!(),
		)?;
		Ok(Self { channel_id: channel_id.into(), handshake })
	}

	pub fn relay<T>(
		&self,
		vm: &mut TestVM<XCVMState<T>>,
		vm_counterparty: &mut TestVM<XCVMState<T>>,
		tx_relayer: BlockchainTransaction,
		tx_relayer_counterparty: BlockchainTransaction,
		gas: u64,
	) -> Result<(Vec<Option<Binary>>, Vec<Event>), TestError> {
		let mut all_datas = Vec::new();
		let mut all_events = Vec::new();
		InMemoryIbcNetwork::new(&mut vm.vm_state, &mut vm_counterparty.vm_state).relay::<()>(
			self.handshake.channel.clone(),
			Env {
				block: tx_relayer.block.clone(),
				transaction: tx_relayer.transaction.clone(),
				contract: ContractInfo { address: vm.xcvm_state.gateway.clone().into() },
			},
			Env {
				block: tx_relayer_counterparty.block.clone(),
				transaction: tx_relayer_counterparty.transaction.clone(),
				contract: ContractInfo {
					address: vm_counterparty.xcvm_state.gateway.clone().into(),
				},
			},
			tx_relayer.info.clone(),
			tx_relayer_counterparty.info.clone(),
			gas,
			&(),
			&(),
			|_, _, _, _| {},
			|(datas, events), _, _, _, _| {
				all_datas.extend(datas);
				all_events.extend(events);
			},
		)?;
		Ok((all_datas, all_events))
	}
}

pub struct TestVM<T> {
	pub network_id: NetworkId,
	pub vm_state: VMState,
	pub xcvm_state: T,
}

impl TestVM<()> {
	pub fn new<N: Network>(contracts: XCVMContracts) -> Self {
		Self::new_with_config::<N>(contracts, |config| config)
	}

	pub fn new_with_config<N: Network>(
		XCVMContracts { interpreter, gateway, cw20 }: XCVMContracts,
		config: impl FnOnce(
			StateBuilder<SubstrateAddressHandler>,
		) -> StateBuilder<SubstrateAddressHandler>,
	) -> Self {
		let vm_state = config(StateBuilder::<SubstrateAddressHandler>::new())
			.add_code(XCVM_INTERPRETER_CODE, interpreter)
			.add_code(XCVM_GATEWAY_CODE, gateway)
			.add_code(XCVM_CW20_CODE, cw20)
			.build();
		Self { network_id: N::ID, vm_state, xcvm_state: () }
	}

	pub fn deploy_xcvm<T>(
		mut self,
		tx: BlockchainTransaction,
	) -> Result<(TestVM<XCVMState<T>>, XCVMDeploymentEvents), TestError> {
		let (gateway_address, (gateway_data, gateway_events)) =
			SubstrateApi::<Dispatch>::instantiate(
				&mut self.vm_state,
				XCVM_GATEWAY_CODE,
				None,
				tx.block,
				tx.transaction,
				tx.info.clone(),
				tx.gas,
				xc_core::gateway::InstantiateMsg(HereItem {
					here_id: self.network_id,
					admin: todo!(),
				}),
			)?;
		Ok((
			TestVM {
				network_id: self.network_id,
				vm_state: self.vm_state,
				xcvm_state: XCVMState::new(gateway_address),
			},
			XCVMDeploymentEvents { gateway_data, gateway_events },
		))
	}
}

impl<T> TestVM<XCVMState<T>> {
	pub fn deploy_asset(
		&mut self,
		tx: BlockchainTransaction,
		initial_balances: impl IntoIterator<Item = Cw20Coin>,
		mint: Option<MinterResponse>,
		asset_id: AssetId,
	) -> Result<(Account, XCVMRegisterAssetEvents), TestError> {
		let (asset_address, (asset_data, asset_events)) = TestApi::instantiate(
			&mut self.vm_state,
			XCVM_CW20_CODE,
			None,
			tx.block.clone(),
			tx.transaction.clone(),
			tx.info.clone(),
			tx.gas,
			cw20_base::msg::InstantiateMsg {
				name: "do not care".into(),
				symbol: "DNC".into(),
				decimals: 7,
				initial_balances: initial_balances.into_iter().collect(),
				mint,
				marketing: None,
			},
		)?;
		let (gateway_data, gateway_events) = TestApi::execute(
			&mut self.vm_state,
			Env {
				block: tx.block.clone(),
				transaction: tx.transaction.clone(),
				contract: ContractInfo { address: self.xcvm_state.gateway.clone().into() },
			},
			tx.info,
			tx.gas,
			xc_core::gateway::ExecuteMsg::Config(ConfigSubMsg::ForceAsset(AssetItem {
				asset_id,
				network_id: todo!("restore"),
				local: xc_core::gateway::AssetReference::Cw20 {
					contract: asset_address.clone().into(),
				},
				bridged: None,
			})),
		)?;
		self.xcvm_state.insert_asset(asset_id, asset_address.clone());
		Ok((
			asset_address,
			XCVMRegisterAssetEvents { asset_data, asset_events, gateway_data, gateway_events },
		))
	}

	pub fn dispatch_program_with_allowance(
		&mut self,
		tx: BlockchainTransaction,
		salt: impl Into<Salt>,
		program: impl Into<XcProgram>,
		assets: impl IntoIterator<Item = (AssetId, u128)>,
		allowance_expires: Option<Expiration>,
	) -> Result<(Option<Binary>, Vec<Event>), TestError> {
		let assets = assets.into_iter().collect::<Vec<_>>();
		for (asset_id, amount) in assets.iter() {
			match self.xcvm_state.assets.get(asset_id) {
				Some(asset_address) => {
					TestApi::execute(
						&mut self.vm_state,
						Env {
							block: tx.block.clone(),
							transaction: tx.transaction.clone(),
							contract: ContractInfo { address: asset_address.clone().into() },
						},
						tx.info.clone(),
						tx.gas,
						&cw20::Cw20ExecuteMsg::IncreaseAllowance {
							spender: self.xcvm_state.gateway.clone().into(),
							amount: (*amount).into(),
							expires: allowance_expires,
						},
					)?;
				},
				None => Err(TestError::AssetNotDeployed)?,
			}
		}
		self.dispatch_program(tx, salt, program, assets)
	}

	pub fn dispatch_program(
		&mut self,
		tx: BlockchainTransaction,
		salt: impl Into<Salt>,
		program: impl Into<XcProgram>,
		assets: impl IntoIterator<Item = (AssetId, u128)>,
	) -> Result<(Option<Binary>, Vec<Event>), TestError> {
		let execute_program = xc_core::gateway::ExecuteProgramMsg {
			salt: salt.into(),
			program: program.into(),
			assets: Funds::from(assets.into_iter().collect::<Vec<_>>()),
		};
		let (data, events) = TestApi::execute(
			&mut self.vm_state,
			Env {
				block: tx.block,
				transaction: tx.transaction,
				contract: ContractInfo { address: self.xcvm_state.gateway.clone().into() },
			},
			tx.info,
			tx.gas,
			xc_core::gateway::ExecuteMsg::ExecuteProgram {
				execute_program,
				tip: self.xcvm_state.gateway.clone().into(),
			},
		)?;
		Ok((data, events))
	}

	pub fn balance_of(
		&mut self,
		asset_id: AssetId,
		tx: BlockchainTransaction,
		account: impl Into<String>,
	) -> Result<cw20::BalanceResponse, TestError> {
		match self.xcvm_state.assets.get(&asset_id) {
			Some(asset_address) => TestQueryApi::query(
				&mut self.vm_state,
				Env {
					block: tx.block,
					transaction: tx.transaction,
					contract: ContractInfo { address: asset_address.clone().into() },
				},
				&cw20::Cw20QueryMsg::Balance { address: account.into() },
			)
			.map_err(Into::into),
			None => Err(TestError::AssetNotDeployed)?,
		}
	}

	pub fn token_info(
		&mut self,
		asset_id: AssetId,
		tx: BlockchainTransaction,
	) -> Result<cw20::TokenInfoResponse, TestError> {
		match self.xcvm_state.assets.get(&asset_id) {
			Some(asset_address) => TestQueryApi::query(
				&mut self.vm_state,
				Env {
					block: tx.block,
					transaction: tx.transaction,
					contract: ContractInfo { address: asset_address.clone().into() },
				},
				&cw20::Cw20QueryMsg::TokenInfo {},
			)
			.map_err(Into::into),
			None => Err(TestError::AssetNotDeployed)?,
		}
	}
}

pub struct XCVMState<T> {
	pub gateway: Account,
	pub assets: HashMap<AssetId, Account>,
	pub custom: HashMap<T, Account>,
}

impl<T> XCVMState<T> {
	pub fn new(gateway: Account) -> Self {
		Self { gateway, assets: Default::default(), custom: Default::default() }
	}

	pub fn insert_asset(&mut self, asset_id: AssetId, address: Account) {
		self.assets.insert(asset_id, address);
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
