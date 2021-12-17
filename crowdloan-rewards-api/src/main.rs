use std::{convert::Infallible, sync::Arc};

use pallet_crowdloan_rewards::models::Proof;
use serde::{Deserialize, Serialize};
use sp_runtime::{AccountId32, MultiSignature};
use subxt_clients::picasso;
use warp::{hyper::StatusCode, Filter};

mod verify;

type PicassoApi = picasso::api::RuntimeApi<picasso::api::DefaultConfig>;

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

fn with_subxt_api(
	api: Arc<PicassoApi>,
) -> impl Filter<Extract = (Arc<PicassoApi>,), Error = Infallible> + Clone {
	warp::any().map(move || api.clone())
}

#[derive(Serialize, Deserialize)]
struct AssociateOrigin {
	// I'm not sure what this should be; should I use Encode/ Decode instead of serde? Or use Encode/ Decode in a custom serde implementation?
// proof: Proof<[u8; 32]>,
// reward_account: RewardAccount
}

async fn associate(
	associate_origin: AssociateOrigin,
	api: Arc<picasso::api::RuntimeApi<picasso::api::DefaultConfig>>,
) -> Result<StatusCode, warp::Rejection> {
	api.tx(); // .what (?)
	Ok(StatusCode::OK)
}
