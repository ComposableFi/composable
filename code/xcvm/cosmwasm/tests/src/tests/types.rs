use cosmwasm_orchestrate::{
	ibc::IbcNetwork,
	vm::{Account, IbcChannelId, State, SubstrateAddressHandler, VmError},
	StateBuilder,
};
use cosmwasm_std::{Env, IbcChannel, IbcOrder, MessageInfo};
use cosmwasm_vm::system::CosmwasmCodeId;
use std::{collections::HashMap, hash::Hash};

pub type VMState = State<(), SubstrateAddressHandler>;

pub type InMemoryIbcNetwork<'a> = IbcNetwork<'a, (), SubstrateAddressHandler>;

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
	code_asset_registry: Vec<u8>,
	code_interpreter: Vec<u8>,
	code_router: Vec<u8>,
	code_gateway: Vec<u8>,
	code_cw20: Vec<u8>,
}

impl XCVMContracts {
	pub fn new(
		code_asset_registry: Vec<u8>,
		code_interpreter: Vec<u8>,
		code_router: Vec<u8>,
		code_gateway: Vec<u8>,
		code_cw20: Vec<u8>,
	) -> Self {
		Self { code_asset_registry, code_interpreter, code_router, code_gateway, code_cw20 }
	}
}

pub struct InMemoryIbcNetworkChannel<'a, T> {
	vm: &'a mut T,
	vm_counterparty: &'a mut T,
	channel_id: IbcChannelId,
	channel: IbcChannel,
}

impl<'a> InMemoryIbcNetworkChannel<'a, VM<()>> {
	pub fn connect<T: Clone + Into<IbcChannelId>>(
		xcvm_contracts: XCVMContracts,
		vm: &'a mut VM<()>,
		vm_counterparty: &'a mut VM<()>,
		channel_id: T,
		connection_id: impl Into<String>,
		version: impl Into<String>,
		ordering: impl Into<IbcOrder>,
		env: impl Into<Env>,
		env_counterparty: impl Into<Env>,
		info: impl Into<MessageInfo>,
		info_counterparty: impl Into<MessageInfo>,
		gas: impl Into<u64>,
	) -> Result<InMemoryIbcNetworkChannel<'a, VM<()>>, TestError> {
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

pub struct VM<T> {
	vm_state: VMState,
	code_asset_registry: CosmwasmCodeId,
	code_interpreter: CosmwasmCodeId,
	code_router: CosmwasmCodeId,
	code_gateway: CosmwasmCodeId,
	code_cw20: CosmwasmCodeId,
	xcvm_state: T,
}

impl VM<()> {
	pub fn new(
		xcvm_contracts: XCVMContracts,
		config: impl FnOnce(
			StateBuilder<SubstrateAddressHandler>,
		) -> StateBuilder<SubstrateAddressHandler>,
	) -> Self {
		let mut vm_state = config(StateBuilder::<SubstrateAddressHandler>::new()).build();
    // vm_state.codes
			// .add_codes(vec![
			// 	&xcvm_contracts.code_asset_registry,
			// 	&xcvm_contracts.code_interpreter,
			// 	&xcvm_contracts.code_router,
			// 	&xcvm_contracts.code_gateway,
			// 	&xcvm_contracts.code_cw20,
			// ]))
			// .build();
		Self { vm_state, xcvm_state: () }
	}
}

pub struct XCVMState<T> {
	pub gateway: Account,
	pub router: Account,
	pub custom: HashMap<T, Account>,
}

impl<T> XCVMState<T> {
	pub const fn new(gateway: Account, router: Account, custom: T) -> Self {
		Self { gateway, router, custom }
	}
}

impl<T> XCVMState<HashMap<T, Account>>
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
