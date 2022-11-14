extern crate prost_build;

fn main() {
	prost_build::compile_protos(&["protos/xcvm_program.proto"], &["protos/"]).unwrap();
}
