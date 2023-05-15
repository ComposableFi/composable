use futures_util::{future::join, StreamExt};

#[tokio::main]
async fn main() {
    async_std::task::spawn(async move { prometheus_sink::main_metrics().unwrap() });

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(
        "info,cranelift_codegen=info,wasmtime_cranelift=info,wasmtime_jit=info",
    ))
    .init();
    let (composable_sender, composable_receiver) = futures_channel::mpsc::unbounded();
    let (composable_requester_sender, composable_requester_receiver) =futures_channel::mpsc::unbounded();
    async_std::task::spawn(async {
        smoldot_source::composable_polkadot(composable_sender, composable_requester_receiver).await;
    });
    let (picasso_sender, picasso_receiver) = futures_channel::mpsc::unbounded();
    let (picasso_requester_sender, picasso_requester_receiver) =futures_channel::mpsc::unbounded();
    async_std::task::spawn(async {
        smoldot_source::picasso_kusama(picasso_sender, picasso_requester_receiver).await;
    });

    let (prometheus_sender, prometheus_receiver) = futures_channel::mpsc::unbounded();
    async_std::task::spawn(async {
        prometheus_sink::main(prometheus_receiver, composable_requester_sender, picasso_requester_sender).await;
    });

    let (a, b) = (prometheus_sender.clone(), prometheus_sender);
    let composable = async_std::task::spawn(async move {
        let mut stream = composable_receiver.enumerate();
        while let Some((_i, events)) = stream.next().await {
            log::debug!("decode");
            let events = subxt_decoder::composable_decoder(events);
            log::info!("launch light clients observers {:?}", events);
            a.unbounded_send(prometheus_sink::ChangeOfInterest::Composable(events))
                .unwrap();
        }
    });
    let picasso = async_std::task::spawn(async move {
        let mut stream = picasso_receiver.enumerate();
        while let Some((_i, events)) = stream.next().await {
            let events = subxt_decoder::picasso_decoder(events);
            log::info!("launch light clients observers  {:?}", events);
            b.unbounded_send(prometheus_sink::ChangeOfInterest::Picasso(events))
                .unwrap();
        }
    });

    join(composable, picasso).await;
}
