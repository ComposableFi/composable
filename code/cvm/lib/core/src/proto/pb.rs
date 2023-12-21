pub mod common {
	include!(concat!(env!("OUT_DIR"), "/cvm.common.rs"));
}
pub mod solidity {
	include!(concat!(env!("OUT_DIR"), "/solidity.rs"));
}
pub mod program {
	include!(concat!(env!("OUT_DIR"), "/cvm.program.rs"));
}
