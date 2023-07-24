extern crate prost_build;

fn main() {
	prost_build::compile_protos(&["protos/xc.proto"], &["protos/"]).expect("compile time");
}
