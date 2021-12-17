use std::{convert::Infallible, sync::Arc};

use pallet_crowdloan_rewards::{
	get_remote_account,
	models::{Proof, RemoteAccount},
};
use sp_core::{Decode, Encode};
use sp_runtime::AccountId32;
use subxt_clients::picasso;
use warp::{hyper::StatusCode, Filter};

type PicassoApi = picasso::api::RuntimeApi<picasso::api::DefaultConfig>;

#[tokio::main]
async fn main() {
	let app = clap::App::new("Crowdloan Rewards API/ Proxy")
		.arg(
			clap::Arg::with_name("prefix")
				.short("p")
				.long("prefix")
				.value_name("PREFIX")
				.help("sets the prefix")
				.takes_value(true),
		)
		.get_matches();

	let prefix = app.value_of("PREFIX").unwrap().as_bytes().into();

	let api = Arc::new(
		subxt::ClientBuilder::new()
			.build()
			.await
			.unwrap()
			.to_runtime_api::<PicassoApi>(),
	);

	let associate_filter = warp::path("associate")
		.and(warp::post())
		.and(warp::body::bytes() /* ::<AssociateOrigin>() */)
		.and(with_subxt_api(api))
		.and(with_prefix(prefix))
		.and_then(associate);

	warp::serve(associate_filter).run(([127, 0, 0, 1], 3030)).await;
}

fn with_subxt_api(
	api: Arc<PicassoApi>,
) -> impl Filter<Extract = (Arc<PicassoApi>,), Error = Infallible> + Clone {
	warp::any().map(move || api.clone())
}

type Prefix = Arc<[u8]>;

fn with_prefix(prefix: Prefix) -> impl Filter<Extract = (Prefix,), Error = Infallible> + Clone {
	warp::any().map(move || prefix.clone())
}

#[derive(Encode, Decode)]
struct AssociateOrigin {
	// I'm not sure what this should be; should I use Encode/ Decode instead of serde? Or use Encode/ Decode in a custom serde implementation?
	proof: Proof<AccountId32>,
	reward_account: RemoteAccount<AccountId32>,
}

async fn associate(
	associate_origin_bytes: warp::hyper::body::Bytes,
	api: Arc<PicassoApi>,
	prefix: Prefix,
) -> Result<StatusCode, warp::Rejection> {
	let associate_origin = match AssociateOrigin::decode(&mut associate_origin_bytes.as_ref()) {
		Ok(ok) => ok,
		// TODO: log error?
		Err(_) => return Ok(StatusCode::BAD_REQUEST),
	};

	// currently errors due to type parameter T
	// get_remote_account(associate_origin.proof, &associate_origin.reward_account, &prefix);
	api.tx(); // .what (?)
	Ok(StatusCode::OK)
}
