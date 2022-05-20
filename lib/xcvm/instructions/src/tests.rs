use crate::{network::XCVMNetwork, protocols::Stableswap, types::AbiEncoded, XCVMContractBuilder};

#[test]
fn test() {
	let _ =
		XCVMContractBuilder::<XCVMNetwork, XCVMInstruction<XCVMNetwork, AbiEncoded, (), ()>>::from(
			XCVMNetwork::PICASSO,
		)
		.call(Stableswap::new((), ()))
		.bridge(XCVMNetwork::ETHEREUM, ())
		.call(Stableswap::new((), ()))
		.transfer((), ());
}
