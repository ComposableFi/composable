use pallet_crowdloan_rewards::models::Proof;
use serde::{Deserialize, Serialize};
use sp_runtime::{AccountId32, MultiSignature};
use warp::{hyper::StatusCode, Filter};

#[tokio::main]
async fn main() {
	let associate_filter = warp::path("associate")
		.and(warp::post())
		.and(warp::body::json::<AssociateOrigin>())
		.and_then(associate);
	// .map(|ao: AssociateOrigin| async move { associate(ao).await })
	// .map(associate);

	warp::serve(associate_filter).run(([127, 0, 0, 1], 3030)).await;
}

mod verify;

use subxt_clients::picasso;

#[derive(Serialize, Deserialize)]
struct AssociateOrigin {
	// proof: Proof<[u8; 32]>,
// reward_account: RewardA
}

async fn associate(associate_origin: AssociateOrigin) -> Result<StatusCode, warp::Rejection> {
	let api = match subxt::ClientBuilder::new().build().await.map_err(|why| why.to_string()) {
		Ok(ok) => ok.to_runtime_api::<picasso::api::RuntimeApi<picasso::api::DefaultConfig>>(),
		Err(why) => return Ok(StatusCode::INTERNAL_SERVER_ERROR),
	};

	

	Ok(StatusCode::OK)
}
