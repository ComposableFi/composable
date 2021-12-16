use serde::{Deserialize, Serialize};
use sp_runtime::{MultiSignature, AccountId32};
use warp::{hyper::StatusCode, Filter};

#[tokio::main]
async fn main() {
	let associate_filter =
		warp::post().and(warp::path("associate")).and(warp::body::json()).map(associate);

	warp::serve(associate_filter).run(([127, 0, 0, 1], 3030)).await;
}

mod verify;

use subxt_clients::picasso;

#[derive(Serialize, Deserialize)]
struct AssociateOrigin {
	signature: MultiSignature,
	public_key: AccountId32
}

fn associate(associate_origin: AssociateOrigin) -> impl warp::Reply {
	StatusCode::OK
}
