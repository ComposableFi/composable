pub mod common {
	include!(concat!(env!("OUT_DIR"), "/cvm.common.rs"));
}
pub mod xcvm {
	include!(concat!(env!("OUT_DIR"), "/cvm.xcvm.rs"));
}
