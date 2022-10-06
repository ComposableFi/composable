use alloc::vec::Vec;
use ibc::core::ics23_commitment::{error::Error, merkle::MerkleProof};
use ibc_proto::ibc::core::commitment::v1::MerkleProof as RawMerkleProof;
use tendermint::merkle::proof::Proof as TendermintProof;

#[allow(unused)]
pub fn convert_tm_to_ics_merkle_proof<H>(
	tm_proof: &TendermintProof,
) -> Result<MerkleProof<H>, Error> {
	let mut proofs = Vec::new();

	for op in &tm_proof.ops {
		let mut parsed = ibc_proto::ics23::CommitmentProof { proof: None };
		prost::Message::merge(&mut parsed, op.data.as_slice())
			.map_err(Error::commitment_proof_decoding_failed)?;

		proofs.push(parsed);
	}

	Ok(MerkleProof::from(RawMerkleProof { proofs }))
}
