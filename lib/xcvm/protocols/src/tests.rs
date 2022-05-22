use alloc::{vec, vec::Vec};
use xcvm_core::{AbiEncoded, XCVMContractBuilder, XCVMInstruction, XCVMNetwork, XCVMProtocol};

use crate::Stableswap;

#[test]
fn test() {
	let contract =
		XCVMContractBuilder::<XCVMNetwork, XCVMInstruction<XCVMNetwork, _, (), ()>>::from(
			XCVMNetwork::PICASSO,
		)
		.call(Stableswap::<()>::new((), ()))
		.bridge(XCVMNetwork::ETHEREUM, ())
		.call(Stableswap::<()>::new((), ()))
		.transfer((), ());

	assert_eq!(
		contract.instructions,
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
