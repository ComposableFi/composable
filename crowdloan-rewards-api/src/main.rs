use std::{sync::Arc, convert::Infallible};

use pallet_crowdloan_rewards::models::Proof;
use serde::{Deserialize, Serialize};
use sp_runtime::{AccountId32, MultiSignature};
use warp::{hyper::StatusCode, Filter};

#[tokio::main]
async fn main() {
	let api = Arc::new(
		subxt::ClientBuilder::new()
			.build()
			.await
			.unwrap()
			.to_runtime_api::<PicassoApi>(),
	);

	let associate_filter = warp::path("associate")
		.and(warp::post())
		.and(warp::body::json::<AssociateOrigin>())
		.and(with_subxt_api(api))
		.and_then(associate);

	warp::serve(associate_filter).run(([127, 0, 0, 1], 3030)).await;
}

type PicassoApi = picasso::api::RuntimeApi<picasso::api::DefaultConfig>;

fn with_subxt_api(
	api: Arc<PicassoApi>,
) -> impl Filter<Extract = (Arc<PicassoApi>,), Error = Infallible> + Clone {
	warp::any().map(move || api.clone())
}

mod verify;

use subxt_clients::picasso;

#[derive(Serialize, Deserialize)]
struct AssociateOrigin {
	// proof: Proof<[u8; 32]>,
// reward_account: RewardA
}

async fn associate(
	associate_origin: AssociateOrigin,
	api: Arc<picasso::api::RuntimeApi<picasso::api::DefaultConfig>>,
) -> Result<StatusCode, warp::Rejection> {
	Ok(StatusCode::OK)
}
