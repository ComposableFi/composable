extern crate prost_build;

fn main() {
	prost_build::compile_protos(
		&["../../../lib/protos/xcvm_program.proto"],
		&["../../../lib/protos/"],
	)
	.unwrap();
}
