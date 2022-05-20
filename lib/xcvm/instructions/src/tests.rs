use crate::{
	instruction::XCVMInstruction, network::XCVMNetwork, protocols::Stableswap, types::AbiEncoded,
	XCVMContractBuilder,
};
use alloc::vec;

#[test]
fn test() {
	let XCVMContractBuilder { network, instructions } = XCVMContractBuilder::<
		XCVMNetwork,
		XCVMInstruction<XCVMNetwork, _, (), ()>,
	>::from(XCVMNetwork::PICASSO)
	.call(Stableswap::new((), ()))
	.bridge(XCVMNetwork::ETHEREUM, ())
	.call(Stableswap::new((), ()))
	.transfer((), ())
	.clone();

	assert_eq!(
		instructions,
		vec![
			XCVMInstruction::Call(AbiEncoded::empty()),
			XCVMInstruction::Bridge(XCVMNetwork::ETHEREUM, ()),
			XCVMInstruction::Call(AbiEncoded::from(vec![
				0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
				0, 0, 0, 0
			])),
			XCVMInstruction::Transfer((), ()),
		]
	);
}
