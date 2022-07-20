use codec::{Decode, Encode};
use parity_wasm::elements::Module;
use scale_info::TypeInfo;
use sp_std::vec::Vec;
use wasm_instrument::gas_metering::{self, MemoryGrowCost, Rules};

/// Errors likely to happen while instrumenting a code.
#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
pub enum InstrumentationError {
	GasMeteringInjection,
	StackHeightLimitingInjection,
}

/// Instrument a code for gas metering and stack height limiting.
pub fn gas_and_stack_instrumentation(
	module: Module,
	gas_module_name: &str,
	stack_limit: u32,
	cost_rules: &impl Rules,
) -> Result<Module, InstrumentationError> {
	let gas_instrumented_module = gas_metering::inject(module, cost_rules, gas_module_name)
		.map_err(|_| InstrumentationError::GasMeteringInjection)?;
	let stack_and_gas_instrumented_module =
		wasm_instrument::inject_stack_limiter(gas_instrumented_module, stack_limit)
			.map_err(|_| InstrumentationError::StackHeightLimitingInjection)?;
	Ok(stack_and_gas_instrumented_module)
}
