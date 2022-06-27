use super::{FeedResult, Feed};
use subxt::{ClientBuilder, DefaultConfig, PolkadotExtrinsicParams};


pub struct ComposableFeed;
use crate::composable_api::self;

impl ComposableFeed {
    pub async fn start() -> FeedResult<Feed<FeedIdentifier, Asset, TimeStampedPrice>> {
        let api = ClientBuilder::new()
            .build()
            .await?
            .to_runtime_api::<composable_api::api::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();
        let mut swapped_events = api
            .events()
            .subscribe()
            .await?
            .filter_events::<composable_api::pablo::events::Swapped,>();

        // process all swapped event
        while let Some(swapped_event) = swapped_events.next().await {
            println!("Swapped Event : {swapped_event:?}");
        }

        Ok(())
    }
}

