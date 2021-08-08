use super::{Feed, FeedNotification, Price, TimeStamped, TimeStampedPrice};
use crate::{
    asset::{AssetPair, VALID_ASSETPAIRS},
    feed::{Exponent, TimeStamp},
};
use chrono::Utc;
use futures::stream::StreamExt;
use jsonrpc_client_transports::{
    transports::ws::connect, RpcError, TypedClient, TypedSubscriptionStream,
};
use serde::{Deserialize, Serialize};
use tokio::{
    sync::mpsc::{self, error::SendError},
    task::JoinHandle,
};
use url::Url;

pub type PythFeedNotification = FeedNotification<AssetPair, TimeStampedPrice>;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum PythSymbolStatus {
    Trading,
    Halted,
    Unknown,
}

#[derive(Copy, Clone, Debug, Deserialize)]
struct PythNotifyPrice {
    status: PythSymbolStatus,
    price: Price,
}

#[derive(Clone, Debug, Deserialize)]
struct PythProductPrice {
    account: String,
    price_exponent: Exponent,
}

#[derive(Clone, Debug, Deserialize)]
struct PythProductAttributes {
    symbol: String,
}

#[derive(Clone, Debug, Deserialize)]
struct PythProduct {
    account: String,
    attr_dict: PythProductAttributes,
    price: Vec<PythProductPrice>,
}

#[derive(Debug)]
pub enum PythError {
    RpcError(RpcError),
    ChannelError(SendError<PythFeedNotification>),
}

#[derive(Serialize)]
struct PythSubscribeParams {
    account: String,
}

pub struct Pyth {
    client: TypedClient,
    handles: Vec<JoinHandle<Result<(), PythError>>>,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum PythNotifyPriceAction {
    YieldFeedNotification(PythFeedNotification),
}

fn notify_price_action(
    asset_pair: &AssetPair,
    product_price: &PythProductPrice,
    notify_price: &PythNotifyPrice,
    timestamp: &TimeStamp,
) -> Option<PythNotifyPriceAction> {
    match notify_price.status {
        PythSymbolStatus::Trading => Some(PythNotifyPriceAction::YieldFeedNotification(
            FeedNotification::PriceUpdated(
                Feed::Pyth,
                *asset_pair,
                TimeStamped {
                    value: (notify_price.price, product_price.price_exponent),
                    timestamp: *timestamp,
                },
            ),
        )),
        PythSymbolStatus::Halted => None,
        PythSymbolStatus::Unknown => None,
    }
}

impl Pyth {
    pub async fn new(url: &url::Url) -> Result<Pyth, PythError> {
        let client = connect::<TypedClient>(url)
            .await
            .map_err(PythError::RpcError)?;
        Ok(Pyth {
            client,
            handles: Vec::new(),
        })
    }

    async fn get_product_list(&self) -> Result<Vec<PythProduct>, PythError> {
        self.client
            .call_method::<(), Vec<PythProduct>>("get_product_list", "", ())
            .await
            .map_err(PythError::RpcError)
    }

    async fn subscribe(
        &mut self,
        output: mpsc::Sender<PythFeedNotification>,
        asset_pair: AssetPair,
        product_price: PythProductPrice,
    ) -> Result<(), PythError> {
        log::info!(
            "Subscribing to asset pair {:?} from account {:?}",
            asset_pair,
            product_price.account
        );
        let mut stream: TypedSubscriptionStream<PythNotifyPrice> = self
            .client
            .subscribe(
                "subscribe_price",
                [PythSubscribeParams {
                    account: product_price.account.to_string(),
                }],
                "notify_price",
                "",
                "",
            )
            .map_err(PythError::RpcError)?;
        let join_handle = tokio::spawn(async move {
            output
                .send(FeedNotification::Opened(Feed::Pyth, asset_pair))
                .await
                .map_err(PythError::ChannelError)?;
            'a: loop {
                match stream.next().await {
                    Some(Ok(notify_price)) => {
                        log::debug!(
                            "received notify_price, {:?}, {:?}",
                            asset_pair,
                            notify_price
                        );
                        let timestamp = TimeStamp(Utc::now().timestamp());
                        match notify_price_action(
                            &asset_pair,
                            &product_price,
                            &notify_price,
                            &timestamp,
                        ) {
                            Some(PythNotifyPriceAction::YieldFeedNotification(
                                feed_notification,
                            )) => {
                                output
                                    .send(feed_notification)
                                    .await
                                    .map_err(PythError::ChannelError)?;
                            }
                            None => {
                                // TODO: should we close the feed if the received price don't yield a price update?
                                // e.g. the SymbolStatus != Trading
                            }
                        }
                    }
                    Some(Err(e)) => {
                        log::error!("unexpected rpc error: {:?}", e);
                        break 'a;
                    }
                    None => break 'a,
                }
            }
            output
                .send(FeedNotification::Closed(Feed::Pyth, asset_pair))
                .await
                .map_err(PythError::ChannelError)?;
            Ok(())
        });
        self.handles.push(join_handle);
        Ok(())
    }

    pub async fn subscribe_to_asset(
        &mut self,
        output: &mpsc::Sender<PythFeedNotification>,
        asset_pair: &AssetPair,
    ) -> Result<(), PythError> {
        let asset_pair_symbol = asset_pair.symbol();
        let product_prices = self
            .get_product_list()
            .await?
            .iter()
            .filter(|p| p.attr_dict.symbol == asset_pair_symbol)
            .flat_map(|p| p.price.clone())
            .collect::<Vec<_>>();
        log::info!("Accounts for {:?}: {:?}", asset_pair_symbol, product_prices);
        for product_price in product_prices {
            self.subscribe(output.clone(), *asset_pair, product_price)
                .await?
        }
        Ok(())
    }

    pub async fn terminate(&self) {
        self.handles.iter().for_each(drop);
    }
}

// TODO: manage multiple feeds
pub async fn run_full_subscriptions(
    pythd_host: &String,
) -> (Pyth, mpsc::Receiver<PythFeedNotification>) {
    /* Its important to drop the initial feed_in as it will be cloned for all subsequent tasks
    The received won't get notified if all cloned senders are closed but not the 'main' one.
     */
    let (feed_in, feed_out) = mpsc::channel::<PythFeedNotification>(128);

    let mut pyth = Pyth::new(&Url::parse(&pythd_host).expect("invalid pythd host address."))
        .await
        .expect("connection to pythd failed");

    // TODO: subscribe to all asset prices? cli configurable?
    log::info!("successfully connected to pythd.");
    for asset_pair in VALID_ASSETPAIRS.iter() {
        pyth.subscribe_to_asset(&feed_in, asset_pair)
            .await
            .expect("failed to subscribe to asset");
    }

    (pyth, feed_out)
}

#[cfg(test)]
mod tests {
    use crate::{asset::*, feed::FeedNotification};

    use super::*;

    #[test]
    fn test_notify_price_action() {
        let account = "irrelevant".to_string();
        let product_price = PythProductPrice {
            account,
            price_exponent: Exponent(0x1337),
        };
        let price = Price(0xCAFEBABE);
        let timestamp = TimeStamp(0xDEADC0DE);
        VALID_ASSETPAIRS.iter().for_each(|asset_pair| {
            [
                (PythSymbolStatus::Halted, None),
                (PythSymbolStatus::Unknown, None),
                (
                    PythSymbolStatus::Trading,
                    Some(PythNotifyPriceAction::YieldFeedNotification(
                        FeedNotification::PriceUpdated(
                            Feed::Pyth,
                            *asset_pair,
                            TimeStamped {
                                value: (price, product_price.price_exponent),
                                timestamp,
                            },
                        ),
                    )),
                ),
            ]
            .iter()
            .for_each(|&(status, expected_action)| {
                let notify_price = PythNotifyPrice { status, price };
                assert_eq!(
                    expected_action,
                    notify_price_action(asset_pair, &product_price, &notify_price, &timestamp)
                )
            });
        });
    }
}
