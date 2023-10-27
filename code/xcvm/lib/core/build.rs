extern crate prost_build;

const PROTOS_DIR: &str = "proto";

fn main() -> std::io::Result<()> {
	let mut files = Vec::new();
	for entry in std::fs::read_dir(PROTOS_DIR)? {
		if let Ok(name) = entry?.file_name().into_string() {
			if !name.starts_with('.') && name.ends_with(".proto") {
				files.push([PROTOS_DIR, "/", name.as_str()].concat())
			}
		}
	}
	prost_build::compile_protos(&files, &[PROTOS_DIR])
}
