use crate::{AbiEncoded, XCVMContractBuilder, XCVMInstruction, XCVMNetwork, XCVMProtocol};
use alloc::vec;

#[test]
fn test() {
	struct DummyProtocol;

	impl XCVMProtocol<XCVMNetwork> for DummyProtocol {
		type Error = ();
		fn serialize(&self, network: XCVMNetwork) -> Result<AbiEncoded, ()> {
			match network {
				XCVMNetwork::PICASSO => Ok(AbiEncoded::empty()),
				XCVMNetwork::ETHEREUM => Ok(AbiEncoded::from(vec![4])),
				_ => Err(()),
			}
		}
	}

	let contract = || -> Result<_, ()> {
		Ok(XCVMContractBuilder::<XCVMNetwork, XCVMInstruction<XCVMNetwork, _, (), ()>>::from(
			XCVMNetwork::PICASSO,
		)
		.call(DummyProtocol)?
		.bridge(XCVMNetwork::ETHEREUM, ())
		.call(DummyProtocol)?
		.transfer((), ()))
	}()
	.expect("valid contract");

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
