use ibc_proto::google::protobuf::Any;
use metrics::handler::MetricsHandler;
use primitives::Chain;

/// This sends messages to the sink chain in a gas-aware manner.
pub async fn flush_message_batch(
	msgs: Vec<Any>,
	metrics: Option<&MetricsHandler>,
	sink: &impl Chain,
) -> Result<(), anyhow::Error> {
	let block_max_weight = sink.block_max_weight();
	let batch_weight = sink.estimate_weight(msgs.clone()).await?;

	if let Some(metrics) = metrics {
		metrics.handle_transaction_costs(batch_weight, &msgs).await;
	}

	let ratio = (batch_weight / block_max_weight) as usize;
	if ratio == 0 {
		sink.submit(msgs).await?;
		return Ok(())
	}

	// whelp our batch exceeds the block max weight.
	let chunk = if ratio == 1 {
		// split the batch into ratio * 2
		ratio * 2
	} else {
		// split the batch into ratio + 2
		ratio + 2
	};

	log::info!(
		"Outgoing messages weight: {} exceeds the block max weight: {}. Chunking {} messages into {} chunks",
        batch_weight, block_max_weight, msgs.len(), chunk,
	);
	// TODO: return number of failed messages and record it to metrics
	for batch in msgs.chunks(chunk) {
		// send out batches.
		sink.submit(batch.to_vec()).await?;
	}

	Ok(())
}
