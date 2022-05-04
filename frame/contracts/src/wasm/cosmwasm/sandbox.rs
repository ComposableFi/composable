use super::types::{Env, ExecuteResult, InstantiateResult, MessageInfo, QueryResult};
use sp_runtime::DispatchError;

pub trait CosmwasmSandbox {
	fn instantiate(
		&mut self,
		env: Env,
		info: MessageInfo,
		message: &[u8],
	) -> Result<InstantiateResult, DispatchError>;

	fn execute(
		&mut self,
		env: Env,
		info: MessageInfo,
		message: &[u8],
	) -> Result<ExecuteResult, DispatchError>;

	fn query(&mut self, env: Env, message: &[u8]) -> Result<QueryResult, DispatchError>;
}
