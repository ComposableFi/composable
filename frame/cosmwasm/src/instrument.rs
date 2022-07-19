use wasm_instrument::{
	gas_metering::{self, MemoryGrowCost, Rules},
	parity_wasm::elements::{Instruction, Module},
};

/// Errors likely to happen while instrumenting a code.
#[derive(Clone, Debug)]
pub enum InstrumentationError {
	Decoding(wasm_instrument::parity_wasm::elements::Error),
	GasMeteringInjection,
	StackHeightInjection,
	Encoding(wasm_instrument::parity_wasm::elements::Error),
}

/// Instrument a code for gas metering and stack height limiting.
pub fn instrument(
	gas_module_name: &str,
	code: &[u8],
	stack_limit: u32,
	cost_rules: &impl Rules,
) -> Result<Vec<u8>, InstrumentationError> {
	let module = Module::from_bytes(code).map_err(InstrumentationError::Decoding)?;
	let gas_instrumented_module = gas_metering::inject(module, cost_rules, gas_module_name)
		.map_err(|_| InstrumentationError::GasMeteringInjection)?;
	let stack_and_gas_instrumented_module =
		wasm_instrument::inject_stack_limiter(gas_instrumented_module, stack_limit)
			.map_err(|_| InstrumentationError::StackHeightInjection)?;
	stack_and_gas_instrumented_module
		.into_bytes()
		.map_err(InstrumentationError::Encoding)
}
