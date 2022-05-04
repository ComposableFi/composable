use super::types::*;
use crate::{exec::Ext, wasm::Runtime};
use sp_runtime::{ArithmeticError, DispatchError};
use sp_sandbox::{ReturnValue, SandboxInstance, SandboxMemory, Value};
use sp_std::{vec, vec::Vec};

/* TODO(hussein-aitlahcen):
   - allow runtime to access to marshalling as we probably need them in host functions (defined in runtime).
   - make error strongly typed
*/

// TODO(hussein-aitlahcen): use thoses constants for export checking
// the doc is unclear though as it state that instantiate/execute/query is mandatory but almost 0 of
// their example satisfy this...
pub const INSTANTIATE_FUNCTION: &str = "instantiate";
pub const EXECUTE_FUNCTION: &str = "execute";
pub const QUERY_FUNCTION: &str = "query";

pub struct CosmwasmInstance<'a, E: Ext + 'a> {
	instance: sp_sandbox::default_executor::Instance<Runtime<'a, E>>,
	runtime: Runtime<'a, E>,
}

impl<'a, E: Ext + 'a> CosmwasmInstance<'a, E> {
	pub fn new(
		instance: sp_sandbox::default_executor::Instance<Runtime<'a, E>>,
		runtime: Runtime<'a, E>,
	) -> Self {
		Self { instance, runtime }
	}

	fn allocate<T: TryInto<i32>>(&mut self, len: T) -> Result<u32, DispatchError> {
		log::debug!(target: "runtime::contracts", "Allocate");
		match self.instance.invoke(
			"allocate",
			&[Value::I32(
				len.try_into()
					.map_err(|_| DispatchError::Arithmetic(ArithmeticError::Overflow))?,
			)],
			&mut self.runtime,
		) {
			Ok(ReturnValue::Value(Value::I32(v))) => Ok(v as u32),
			e => {
				log::debug!(target: "runtime::contracts", "Allocate failed: {:?}", e);
				Err(DispatchError::Other("allocate failed"))
			},
		}
	}

	fn deallocate<T: TryInto<i32>>(&mut self, ptr: T) -> Result<(), DispatchError> {
		log::debug!(target: "runtime::contracts", "Deallocate");
		match self.instance.invoke(
			"deallocate",
			&[Value::I32(
				ptr.try_into()
					.map_err(|_| DispatchError::Arithmetic(ArithmeticError::Overflow))?,
			)],
			&mut self.runtime,
		) {
			Ok(ReturnValue::Unit) => Ok(()),
			e => {
				log::debug!(target: "runtime::contracts", "Deallocate failed: {:?}", e);
				Err(DispatchError::Other("deallocate failed"))
			},
		}
	}

	fn passthrough_in(&mut self, data: &[u8]) -> Result<u32, DispatchError> {
		let ptr = self.allocate(data.len())?;
		self.runtime
			.memory()
			.write_region(ptr, &data)
			.map_err(|_| DispatchError::Other("could not write region"))?;
		Ok(ptr)
	}

	fn marshall_in<T>(&mut self, x: &T) -> Result<u32, DispatchError>
	where
		T: serde::ser::Serialize + ?Sized,
	{
		let serialized =
			serde_json::to_vec(x).map_err(|_| DispatchError::Other("couldn't serialize"))?;
		self.passthrough_in(&serialized)
	}

	fn marshall_out<T>(&mut self, ptr: u32) -> Result<T, DispatchError>
	where
		T: serde::de::DeserializeOwned + DeserializeLimit + ?Sized,
	{
		log::debug!(target: "runtime::contracts", "Marshall out");
		let value = self
			.runtime
			.memory()
			.read_region(ptr, T::deserialize_limit())
			.map_err(|_| DispatchError::Other("could not read region"))?;
		serde_json::from_slice(&value).map_err(|_| DispatchError::Other("couldn't deserialize"))
	}

	// TODO(hussein-aitlahcen): refactor instantiate/execute/query with a generic function
	pub fn instantiate(
		&mut self,
		env: Env,
		info: MessageInfo,
		message: &[u8],
	) -> Result<InstantiateResult, DispatchError> {
		let parameters =
			vec![self.marshall_in(&env)?, self.marshall_in(&info)?, self.passthrough_in(message)?]
				.into_iter()
				.map(|v| Value::I32(v as i32))
				.collect::<Vec<_>>();
		let result = self.instance.invoke(INSTANTIATE_FUNCTION, &parameters, &mut self.runtime);
		match result {
			Ok(ReturnValue::Value(Value::I32(response_ptr))) => {
				log::debug!(target: "runtime::contracts", "Instantiate done {:?}", result);
				let response = self.marshall_out::<InstantiateResult>(response_ptr as u32);
				self.deallocate(response_ptr)?;
				response
			},
			e => {
				log::debug!(target: "runtime::contracts", "Instantiate failed {:?}", e);
				Err(DispatchError::Other("could not instantiate"))
			},
		}
	}

	pub fn execute(
		&mut self,
		env: Env,
		info: MessageInfo,
		message: &[u8],
	) -> Result<ExecuteResult, DispatchError> {
		let parameters =
			vec![self.marshall_in(&env)?, self.marshall_in(&info)?, self.passthrough_in(message)?]
				.into_iter()
				.map(|v| Value::I32(v as i32))
				.collect::<Vec<_>>();
		let result = self.instance.invoke(EXECUTE_FUNCTION, &parameters, &mut self.runtime);
		match result {
			Ok(ReturnValue::Value(Value::I32(response_ptr))) => {
				log::debug!(target: "runtime::contracts", "Execute done {:?}", result);
				let response = self.marshall_out::<ExecuteResult>(response_ptr as u32);
				self.deallocate(response_ptr)?;
				response
			},
			e => {
				log::debug!(target: "runtime::contracts", "Execute failed {:?}", e);
				Err(DispatchError::Other("could not execute"))
			},
		}
	}

	pub fn query(&mut self, env: Env, message: &[u8]) -> Result<QueryResult, DispatchError> {
		let parameters = vec![self.marshall_in(&env)?, self.passthrough_in(message)?]
			.into_iter()
			.map(|v| Value::I32(v as i32))
			.collect::<Vec<_>>();
		let result = self.instance.invoke(QUERY_FUNCTION, &parameters, &mut self.runtime);
		match result {
			Ok(ReturnValue::Value(Value::I32(response_ptr))) => {
				log::debug!(target: "runtime::contracts", "Instantiate done {:?}", result);
				let response = self.marshall_out::<QueryResult>(response_ptr as u32);
				self.deallocate(response_ptr)?;
				response
			},
			e => {
				log::debug!(target: "runtime::contracts", "Query failed {:?}", e);
				Err(DispatchError::Other("could not query"))
			},
		}
	}
}
