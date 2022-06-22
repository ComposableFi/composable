use chain::Chain;
use futures::StreamExt;

mod chain;

/// Core relayer loop, waits for new finality events and forwards any new [`ibc::IbcEvents`]
/// to the counter party chain.
pub async fn relay(chain_a: impl Chain, chain_b: impl Chain) {
    let (mut chain_a_finality, mut chain_b_finality) = (
        chain_a.finality_notifications().await,
        chain_b.finality_notifications().await,
    );
    loop {
        tokio::select! {
            // new finality event from chain A
            result = chain_a_finality.next() => {
                match result {
                    // stream closed
                    None => break,
                    Some(()) => {
                        if let Some(events) = chain_a.query_latest_ibc_events().await {
                            chain_b.submit_ibc_events(events).await;
                        }
                    }
                }
            },
            // new finality event from chain B
            result = chain_b_finality.next() => {
                match result {
                   // stream closed
                    None => break,
                    Some(()) => {
                        if let Some(events) = chain_b.query_latest_ibc_events().await {
                            chain_a.submit_ibc_events(events).await;
                        }
                    }
                }
            }
        }
    }
}
