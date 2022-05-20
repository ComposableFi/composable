use crate::{AbiEncoded, XCVMContractBuilder, XCVMInstruction, XCVMNetwork, XCVMProtocol};
use alloc::vec;

#[test]
fn test() {
	struct DummyProtocol;

	impl XCVMProtocol<XCVMNetwork, AbiEncoded> for DummyProtocol {
		fn serialize(&self, network: XCVMNetwork) -> AbiEncoded {
			match network {
				XCVMNetwork::PICASSO => AbiEncoded::empty(),
				XCVMNetwork::ETHEREUM => AbiEncoded::from(vec![4]),
				_ => todo!("handle error of invalid network id"),
			}
		}
	}

	let contract =
		XCVMContractBuilder::<XCVMNetwork, XCVMInstruction<XCVMNetwork, _, (), ()>>::from(
			XCVMNetwork::PICASSO,
		)
		.call(DummyProtocol)
		.bridge(XCVMNetwork::ETHEREUM, ())
		.call(DummyProtocol)
		.transfer((), ());

	assert_eq!(
		contract.instructions,
		vec![
			XCVMInstruction::Call(AbiEncoded::empty()),
			XCVMInstruction::Bridge(XCVMNetwork::ETHEREUM, ()),
			XCVMInstruction::Call(AbiEncoded::from(vec![4])),
			XCVMInstruction::Transfer((), ()),
		]
	);
}
