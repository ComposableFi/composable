include!(concat!(env!("OUT_DIR"), "/parachain.rs"));
use api::runtime_types::{dali_runtime::Call, frame_system::extensions::check_nonce::CheckNonce};
use codec::{Compact, Decode, Encode, Input};
use sp_runtime::generic::Era;
use subxt::Config;

#[derive(Encode, Decode, Clone, Eq, PartialEq)]
pub struct ChargeAssetTxPayment {
	#[codec(compact)]
	tip: u128,
	asset_id: Option<u128>,
}

pub type SignedExtra = (Era, CheckNonce, ChargeAssetTxPayment);

pub struct UncheckedExtrinsic<T: Config> {
	pub signature: Option<(<T as Config>::Address, sp_runtime::MultiSignature, SignedExtra)>,
	pub function: Call,
}

// the code was taken from https://github.com/paritytech/substrate/blob/0c1ccdaa53556a106aa69c23f19527e435970237/primitives/runtime/src/generic/unchecked_extrinsic.rs#L233
impl<T: Config> Decode for UncheckedExtrinsic<T> {
	fn decode<I: Input>(input: &mut I) -> Result<Self, codec::Error> {
		const EXTRINSIC_FORMAT_VERSION: u8 = 4;

		// This is a little more complicated than usual since the binary format must be compatible
		// with SCALE's generic `Vec<u8>` type. Basically this just means accepting that there
		// will be a prefix of vector length.
		let expected_length: Compact<u32> = Decode::decode(input)?;
		let before_length = input.remaining_len()?;

		let version = input.read_byte()?;

		let is_signed = version & 0b1000_0000 != 0;
		let version = version & 0b0111_1111;
		if version != EXTRINSIC_FORMAT_VERSION {
			return Err("Invalid transaction version".into())
		}

		let signature = is_signed.then(|| Decode::decode(input)).transpose()?;
		let function = Decode::decode(input)?;

		if let Some((before_length, after_length)) =
			input.remaining_len()?.and_then(|a| before_length.map(|b| (b, a)))
		{
			let length = before_length.saturating_sub(after_length);

			if length != expected_length.0 as usize {
				return Err("Invalid length prefix".into())
			}
		}

		Ok(Self { signature, function })
	}
}
