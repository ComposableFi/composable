use super::{Feed, FeedNotification, Price, TimeStamped};
use crate::{
    asset::{to_symbol, AssetPair},
    feed::{Exponent, TimeStamp},
};
use chrono::Utc;
use jsonrpc_client_transports::{
    transports::ws::connect, RpcError, TypedClient, TypedSubscriptionStream,
};
use jsonrpc_core_client::futures::StreamExt;
use serde::{Deserialize, Serialize};
use tokio::{
    sync::mpsc::{self, error::SendError},
    task::JoinHandle,
};

#[derive(Debug, Deserialize)]
struct PythNotification {
    price: u64,
}

#[derive(Clone, Debug, Deserialize)]
struct PythProductPrice {
    account: String,
    price_exponent: i32,
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
    ChannelError(SendError<FeedNotification>),
}

#[derive(Serialize)]
struct PythSubscribeParams {
    account: String,
}

pub struct Pyth {
    client: TypedClient,
    handles: Vec<JoinHandle<Result<(), PythError>>>,
}

impl Pyth {
    pub async fn new(url: &url::Url) -> Result<Pyth, PythError> {
        let client = connect::<TypedClient>(url)
            .await
            .map_err(|e| PythError::RpcError(e))?;
        Ok(Pyth {
            client,
            handles: Vec::new(),
        })
    }

    async fn get_product_list(&self) -> Result<Vec<PythProduct>, PythError> {
        self.client
            .call_method::<(), Vec<PythProduct>>("get_product_list", "", ())
            .await
            .map_err(|e| PythError::RpcError(e))
    }

    async fn subscribe(
        &mut self,
        output: mpsc::Sender<FeedNotification>,
        asset_pair: AssetPair,
        product_price: PythProductPrice,
    ) -> Result<(), PythError> {
        log::info!(
            "Subscribing to asset pair {:?} from account {:?}",
            asset_pair,
            product_price.account
        );
        let mut stream: TypedSubscriptionStream<PythNotification> = self
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
            .map_err(|e| PythError::RpcError(e))?;
        let join_handle = tokio::spawn(async move {
            output
                .send(FeedNotification::Opened(Feed::Pyth, asset_pair))
                .await
                .map_err(|e| PythError::ChannelError(e))?;
            'a: loop {
                match stream.next().await {
                    Some(notification) => match notification {
                        Ok(price_notification) => {
                            log::debug!(
                                "received price, {:?}, {:?}",
                                asset_pair,
                                price_notification
                            );
                            output
                                .send(FeedNotification::PriceUpdated(
                                    Feed::Pyth,
                                    asset_pair,
                                    TimeStamped {
                                        value: (
                                            Price(price_notification.price),
                                            Exponent(product_price.price_exponent),
                                        ),
                                        timestamp: TimeStamp(Utc::now().timestamp()),
                                    },
                                ))
                                .await
                                .map_err(|e| PythError::ChannelError(e))?;
                        }
                        _ => {
                            log::error!("invalid notification?: {:?}", notification);
                        }
                    },
                    None => break 'a,
                }
            }
            output
                .send(FeedNotification::Closed(Feed::Pyth, asset_pair))
                .await
                .map_err(|e| PythError::ChannelError(e))?;
            Ok(())
        });
        self.handles.push(join_handle);
        Ok(())
    }

    pub async fn subscribe_to_asset(
        &mut self,
        output: &mpsc::Sender<FeedNotification>,
        asset_pair: &AssetPair,
    ) -> Result<(), PythError> {
        let asset_pair_symbol = to_symbol(asset_pair);
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
