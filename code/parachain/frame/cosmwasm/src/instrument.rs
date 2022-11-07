use crate::{weights::WeightInfo, Config};
use codec::{Decode, Encode};
use core::marker::PhantomData;
use frame_support::weights::Weight;
use parity_wasm::elements::{Instruction, Module};
use scale_info::TypeInfo;
use wasm_instrument::gas_metering::{self, MemoryGrowCost, Rules};

pub const INSTRUCTIONS_MULTIPLIER: u32 = 100;

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

type WeightFn = fn(n: u32) -> Weight;

/// Calculates and returns the weight of a single instruction
///
/// * weight_fn: Generated weight function of an instruction
/// * additional_instrs: Count of instructions that are used to be able to generate a valid program
///   but should be included in the weight calculation
fn calculate_weight<T: Config>(weight_fn: WeightFn, n_additional_instrs: u32) -> u32 {
	((weight_fn(1) - weight_fn(0)) as u32 / INSTRUCTIONS_MULTIPLIER) -
		(((T::WeightInfo::instruction_I64Const(1) - T::WeightInfo::instruction_I64Const(0))
			as u32 / 100 / 2) *
			n_additional_instrs)
}

/// Calculates a weight that is dependent on other weight. Eg. `else` because it cannot
/// exist without an `if`
fn calculate_weight_custom<T: Config>(weight_fn: WeightFn, custom_fn: WeightFn) -> u32 {
	((weight_fn(1) - weight_fn(0)) - (custom_fn(1) - custom_fn(0))) as u32 / INSTRUCTIONS_MULTIPLIER
}

#[derive(Encode, Decode, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct CostRules<T: Config> {
	i64const: u32,
	f64const: u32,
	i64load: u32,
	f64load: u32,
	i64store: u32,
	f64store: u32,
	i64eq: u32,
	i64eqz: u32,
	i64ne: u32,
	i64lts: u32,
	i64gts: u32,
	i64gtu: u32,
	i64les: u32,
	i64ges: u32,
	i64leu: u32,
	i64geu: u32,
	i64clz: u32,
	i64ctz: u32,
	i64popcnt: u32,
	i64add: u32,
	i64mul: u32,
	i64divs: u32,
	i64divu: u32,
	i64rems: u32,
	i64remu: u32,
	i64and: u32,
	i64or: u32,
	i64xor: u32,
	i64shl: u32,
	i64shru: u32,
	i64rotl: u32,
	i64rotr: u32,
	f64eq: u32,
	f64ne: u32,
	f64lt: u32,
	f64gt: u32,
	f64le: u32,
	f64ge: u32,
	f64abs: u32,
	f64neg: u32,
	f64ceil: u32,
	f64floor: u32,
	f64trunc: u32,
	f64nearest: u32,
	f64sqrt: u32,
	f64add: u32,
	f64sub: u32,
	f64mul: u32,
	f64div: u32,
	f64min: u32,
	f64max: u32,
	f64copysign: u32,
	select: u32,
	if_: u32,
	else_: u32,
	getlocal: u32,
	setlocal: u32,
	teelocal: u32,
	setglobal: u32,
	getglobal: u32,
	currentmemory: u32,
	growmemory: u32,
	#[codec(skip)]
	_marker: PhantomData<T>,
}

impl<T: Config> Default for CostRules<T> {
	fn default() -> Self {
		Self {
			i64const: calculate_weight::<T>(T::WeightInfo::instruction_I64Const, 1),
			f64const: calculate_weight::<T>(T::WeightInfo::instruction_F64Const, 1),
			i64load: calculate_weight::<T>(T::WeightInfo::instruction_I64Load, 2),
			f64load: calculate_weight::<T>(T::WeightInfo::instruction_F64Load, 2),
			i64store: calculate_weight::<T>(T::WeightInfo::instruction_I64Store, 2),
			f64store: calculate_weight::<T>(T::WeightInfo::instruction_F64Store, 2),
			i64eq: calculate_weight::<T>(T::WeightInfo::instruction_I64Eq, 3),
			i64eqz: calculate_weight::<T>(T::WeightInfo::instruction_I64Eqz, 2),
			i64ne: calculate_weight::<T>(T::WeightInfo::instruction_I64Ne, 3),
			i64lts: calculate_weight::<T>(T::WeightInfo::instruction_I64LtS, 3),
			i64gts: calculate_weight::<T>(T::WeightInfo::instruction_I64GtS, 3),
			i64gtu: calculate_weight::<T>(T::WeightInfo::instruction_I64GtU, 3),
			i64les: calculate_weight::<T>(T::WeightInfo::instruction_I64LeS, 3),
			i64ges: calculate_weight::<T>(T::WeightInfo::instruction_I64GeS, 3),
			i64leu: calculate_weight::<T>(T::WeightInfo::instruction_I64LeU, 3),
			i64geu: calculate_weight::<T>(T::WeightInfo::instruction_I64GeU, 3),
			i64clz: calculate_weight::<T>(T::WeightInfo::instruction_I64Clz, 3),
			i64ctz: calculate_weight::<T>(T::WeightInfo::instruction_I64Ctz, 2),
			i64popcnt: calculate_weight::<T>(T::WeightInfo::instruction_I64Popcnt, 2),
			i64add: calculate_weight::<T>(T::WeightInfo::instruction_I64Add, 3),
			i64mul: calculate_weight::<T>(T::WeightInfo::instruction_I64Mul, 3),
			i64divs: calculate_weight::<T>(T::WeightInfo::instruction_I64DivS, 3),
			i64divu: calculate_weight::<T>(T::WeightInfo::instruction_I64DivU, 3),
			i64rems: calculate_weight::<T>(T::WeightInfo::instruction_I64RemS, 3),
			i64remu: calculate_weight::<T>(T::WeightInfo::instruction_I64RemU, 3),
			i64and: calculate_weight::<T>(T::WeightInfo::instruction_I64And, 3),
			i64or: calculate_weight::<T>(T::WeightInfo::instruction_I64Or, 3),
			i64xor: calculate_weight::<T>(T::WeightInfo::instruction_I64Xor, 3),
			i64shl: calculate_weight::<T>(T::WeightInfo::instruction_I64Shl, 3),
			i64shru: calculate_weight::<T>(T::WeightInfo::instruction_I64ShrU, 3),
			// TODO(aeryz): I64ShrS
			i64rotl: calculate_weight::<T>(T::WeightInfo::instruction_I64Rotl, 3),
			i64rotr: calculate_weight::<T>(T::WeightInfo::instruction_I64Rotr, 3),
			f64eq: calculate_weight::<T>(T::WeightInfo::instruction_F64Eq, 3),
			f64ne: calculate_weight::<T>(T::WeightInfo::instruction_F64Ne, 3),
			f64lt: calculate_weight::<T>(T::WeightInfo::instruction_F64Lt, 3),
			f64gt: calculate_weight::<T>(T::WeightInfo::instruction_F64Gt, 3),
			f64le: calculate_weight::<T>(T::WeightInfo::instruction_F64Le, 3),
			f64ge: calculate_weight::<T>(T::WeightInfo::instruction_F64Ge, 3),
			f64abs: calculate_weight::<T>(T::WeightInfo::instruction_F64Abs, 2),
			f64neg: calculate_weight::<T>(T::WeightInfo::instruction_F64Neg, 2),
			f64ceil: calculate_weight::<T>(T::WeightInfo::instruction_F64Ceil, 2),
			f64floor: calculate_weight::<T>(T::WeightInfo::instruction_F64Floor, 2),
			f64trunc: calculate_weight::<T>(T::WeightInfo::instruction_F64Trunc, 2),
			f64nearest: calculate_weight::<T>(T::WeightInfo::instruction_F64Nearest, 2),
			f64sqrt: calculate_weight::<T>(T::WeightInfo::instruction_F64Sqrt, 2),
			f64add: calculate_weight::<T>(T::WeightInfo::instruction_F64Add, 3),
			f64sub: calculate_weight::<T>(T::WeightInfo::instruction_F64Sub, 3),
			f64mul: calculate_weight::<T>(T::WeightInfo::instruction_F64Mul, 3),
			f64div: calculate_weight::<T>(T::WeightInfo::instruction_F64Div, 3),
			f64min: calculate_weight::<T>(T::WeightInfo::instruction_F64Min, 3),
			f64max: calculate_weight::<T>(T::WeightInfo::instruction_F64Max, 3),
			f64copysign: calculate_weight::<T>(T::WeightInfo::instruction_F64Copysign, 3),
			select: calculate_weight::<T>(T::WeightInfo::instruction_Select, 4),
			if_: calculate_weight::<T>(T::WeightInfo::instruction_If, 2),
			else_: calculate_weight_custom::<T>(
				T::WeightInfo::instruction_Else,
				T::WeightInfo::instruction_If,
			),
			getlocal: calculate_weight::<T>(T::WeightInfo::instruction_GetLocal, 1),
			setlocal: calculate_weight::<T>(T::WeightInfo::instruction_SetLocal, 1),
			teelocal: calculate_weight::<T>(T::WeightInfo::instruction_TeeLocal, 2),
			getglobal: calculate_weight::<T>(T::WeightInfo::instruction_GetGlobal, 1),
			setglobal: calculate_weight::<T>(T::WeightInfo::instruction_SetGlobal, 2),
			currentmemory: calculate_weight::<T>(T::WeightInfo::instruction_CurrentMemory, 2),
			growmemory: calculate_weight::<T>(T::WeightInfo::instruction_GrowMemory, 2),
			_marker: PhantomData,
		}
	}
}

impl<T: Config> Rules for CostRules<T> {
	/// Returns the cost for the passed `instruction`.
	fn instruction_cost(&self, instruction: &Instruction) -> Option<u32> {
		let weight = match instruction {
			Instruction::I64Const(_) => self.i64const,
			Instruction::F64Const(_) => self.f64const,
			Instruction::I64Load(_, _) => self.i64load,
			Instruction::F64Load(_, _) => self.f64load,
			Instruction::I64Store(_, _) => self.i64store,
			Instruction::F64Store(_, _) => self.f64store,
			Instruction::I64Eq => self.i64eq,
			Instruction::I64Eqz => self.i64eqz,
			Instruction::I64Ne => self.i64ne,
			Instruction::I64LtS => self.i64lts,
			Instruction::I64GtS => self.i64gts,
			Instruction::I64GtU => self.i64gtu,
			Instruction::I64LeS => self.i64les,
			Instruction::I64GeS => self.i64ges,
			Instruction::I64LeU => self.i64leu,
			Instruction::I64GeU => self.i64geu,
			Instruction::I64Clz => self.i64clz,
			Instruction::I64Ctz => self.i64ctz,
			Instruction::I64Popcnt => self.i64popcnt,
			Instruction::I64Add => self.i64add,
			Instruction::I64Mul => self.i64mul,
			Instruction::I64DivS => self.i64divs,
			Instruction::I64DivU => self.i64divu,
			Instruction::I64RemS => self.i64rems,
			Instruction::I64RemU => self.i64remu,
			Instruction::I64And => self.i64and,
			Instruction::I64Or => self.i64or,
			Instruction::I64Xor => self.i64xor,
			Instruction::I64Shl => self.i64shl,
			Instruction::I64ShrU => self.i64shru,
			// TODO(aeryz): I64ShrS
			Instruction::I64Rotl => self.i64rotl,
			Instruction::I64Rotr => self.i64rotr,
			Instruction::F64Eq => self.f64eq,
			Instruction::F64Ne => self.f64ne,
			Instruction::F64Lt => self.f64lt,
			Instruction::F64Gt => self.f64gt,
			Instruction::F64Le => self.f64le,
			Instruction::F64Ge => self.f64ge,
			Instruction::F64Abs => self.f64abs,
			Instruction::F64Neg => self.f64neg,
			Instruction::F64Ceil => self.f64ceil,
			Instruction::F64Floor => self.f64floor,
			Instruction::F64Trunc => self.f64trunc,
			Instruction::F64Nearest => self.f64nearest,
			Instruction::F64Sqrt => self.f64sqrt,
			Instruction::F64Add => self.f64add,
			Instruction::F64Sub => self.f64sub,
			Instruction::F64Mul => self.f64mul,
			Instruction::F64Div => self.f64div,
			Instruction::F64Min => self.f64min,
			Instruction::F64Max => self.f64max,
			Instruction::F64Copysign => self.f64copysign,
			Instruction::Select => self.select,
			Instruction::If(_) => self.if_,
			Instruction::Else => self.else_,
			Instruction::GetLocal(_) => self.getlocal,
			Instruction::SetLocal(_) => self.setlocal,
			Instruction::TeeLocal(_) => self.teelocal,
			Instruction::GetGlobal(_) => self.getglobal,
			Instruction::SetGlobal(_) => self.setglobal,
			Instruction::CurrentMemory(_) => self.currentmemory,
			Instruction::GrowMemory(_) => self.growmemory,
			_ => 1_000,
		};
		Some(weight)
	}

	/// Returns the costs for growing the memory using the `memory.grow` instruction.
	fn memory_grow_cost(&self) -> MemoryGrowCost {
		// GrowMemory is already benchmarked
		MemoryGrowCost::Free
	}
}
