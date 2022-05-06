fn main() {
	// NOTE: could make reproducible clean builds like next:
	// - remove this file
	// - put relevant rust code behind cfg target wasm into this package
	// - create separate native package which will include bytes of WASM_BINARY (make it read from
	//   target depending on cfg of build debug/release)
	// - make cargo make to build this package for wasm target with all proper env vars
	// - make sure that CI/CD gets wasm only via make file to make builds more reproducible
	// - document that maximal reproducability is possible if to build in layered docker.
	#[cfg(feature = "wasm-builder")]
	{
		substrate_wasm_builder::WasmBuilder::new()
			.with_current_project()
			.export_heap_base()
			.import_memory()
			.build()
	}
}
