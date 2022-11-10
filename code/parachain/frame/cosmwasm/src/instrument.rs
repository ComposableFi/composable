use codec::{Decode, Encode};
use parity_wasm::elements::Module;
use scale_info::TypeInfo;
use wasm_instrument::gas_metering::{self, Rules};

/// Current instrumentation version
/// Must be incremented whenever the instrumentation is updated.
pub const INSTRUMENTATION_VERSION: u16 = 1;

/// Errors that can possibly happen while instrumenting a code.
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
		.map_err(|e| {
			log::debug!(target: "runtime::contracts", "gas_and_stack_instrumentation: {:?}", e);
			InstrumentationError::GasMeteringInjection
		})?;
	let stack_and_gas_instrumented_module =
		wasm_instrument::inject_stack_limiter(gas_instrumented_module, stack_limit).map_err(
			|e| {
				log::debug!(target: "runtime::contracts", "gas_and_stack_instrumentation: {:?}", e);
				InstrumentationError::StackHeightLimitingInjection
			},
		)?;
	Ok(stack_and_gas_instrumented_module)
}
