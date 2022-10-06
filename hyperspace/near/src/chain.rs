use std::pin::Pin;

use futures::Stream;
use ibc::core::ics03_connection::msgs::{conn_open_ack, conn_open_init};
use ibc_proto::google::protobuf::Any;
use near_primitives::{hash::CryptoHash, transaction::Transaction};
use primitives::{Chain, IbcProvider};
use tokio_stream::wrappers::ReceiverStream;

use super::error::Error;
use crate::Client;

#[async_trait::async_trait]
impl Chain for Client {
	async fn finality_notifications(
		&self,
	) -> Pin<Box<dyn Stream<Item = <Self as IbcProvider>::FinalityEvent> + Send + Sync>> {
		let stream = self.indexer.streamer();
		Box::pin(ReceiverStream::new(stream))
	}

	async fn submit_ibc_messages(&self, mut messages: Vec<Any>) -> Result<(), Error> {
		let update_client_message = messages.remove(0);
		let mut permissioned_messages = vec![];
		let mut non_permissioned_messages = vec![];

		for msg in messages {
			if matches!(msg.type_url.as_str(), conn_open_init::TYPE_URL | conn_open_ack::TYPE_URL) {
				permissioned_messages.push(msg)
			} else {
				non_permissioned_messages.push(msg)
			}
		}

		let mut messages = vec![update_client_message];
		if !non_permissioned_messages.is_empty() {
			messages.extend(non_permissioned_messages.into_iter())
		}

		let transaction = Transaction::new(
			self.signer.clone(),
			self.public_key(),
			self.contract_id.clone(),
			0,
			CryptoHash::default(),
		);

		let gas = 1_000_000_000_000;
		let deliver_tx = transaction.clone().function_call(
			"deliver".to_owned(),
			serde_json::to_vec(&messages)?,
			gas,
			0,
		);
		// TODO: handle intermediate receipts
		let _ = self.send_transaction(deliver_tx).await?;

		if !permissioned_messages.is_empty() {
			let deliver_permissioned_tx = transaction.clone().function_call(
				"deliver_permissioned".to_owned(),
				serde_json::to_vec(&permissioned_messages)?,
				gas,
				0,
			);
			let _ = self.send_transaction(deliver_permissioned_tx).await?;
		}
		Ok(())
	}
}
