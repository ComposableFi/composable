use std::io::Result;
fn main() -> Result<()> {
	prost_build::Config::new()
		.btree_map(&["."])
		.out_dir("src/")
		.compile_protos(&["src/xcvm.proto"], &["src/"])
}
