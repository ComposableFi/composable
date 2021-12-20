use std::{convert::Infallible, sync::Arc};

use pallet_crowdloan_rewards::{
	ethereum_recover,
	models::{Proof, RemoteAccount},
	verify_relay,
};
use sp_core::{Decode, Encode};
use sp_runtime::AccountId32;
use subxt_clients::picasso;
use warp::{hyper::StatusCode, Filter};

use crate::contributors::{get_contributors, Contributors};

mod contributors;

type Prefix = Arc<[u8]>;
type PicassoApi = picasso::api::RuntimeApi<picasso::api::DefaultConfig>;

#[tokio::main]
async fn main() {
	let app = clap::App::new("Crowdloan Reward Proxy API")
		.arg(
			clap::Arg::with_name("prefix")
				.short("p")
				.long("prefix")
				.value_name("PREFIX")
				.required(true)
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

	let contributors = Arc::new(get_contributors());

	let associate_filter = warp::path("associate")
		.and(warp::post())
		.and(warp::body::bytes())
		.and(with_data::<Arc<PicassoApi>>(api))
		.and(with_data::<Prefix>(prefix))
		.and(with_data::<Arc<Contributors>>(contributors))
		.and_then(associate);

	warp::serve(associate_filter).run(([127, 0, 0, 1], 3030)).await;
}

pub(crate) fn with_data<T: Clone + Send>(
	data: T,
) -> impl Filter<Extract = (T,), Error = Infallible> + Clone {
	warp::any().map(move || data.clone())
}

#[derive(Encode, Decode)]
struct AssociateOrigin {
	proof: Proof<AccountId32>,
	reward_account: RemoteAccount<AccountId32>,
}

async fn associate(
	associate_origin_bytes: warp::hyper::body::Bytes,
	api: Arc<PicassoApi>,
	prefix: Prefix,
	contributors: Arc<Contributors>,
) -> Result<StatusCode, warp::Rejection> {
	let associate_origin = match AssociateOrigin::decode(&mut associate_origin_bytes.as_ref()) {
		Ok(ok) => ok,
		// TODO: log error?
		Err(_) => return Ok(StatusCode::BAD_REQUEST),
	};

	let remote_account = match associate_origin.proof {
		Proof::Ethereum(eth_proof) => {
			let reward_account_encoded = associate_origin
				.reward_account
				.using_encoded(|x| hex::encode(x).as_bytes().to_vec());
			let ethereum_address =
				match ethereum_recover(&prefix, &reward_account_encoded, &eth_proof) {
					Some(ethereum_address) => ethereum_address,
					None => return Ok(StatusCode::BAD_REQUEST),
				};

			RemoteAccount::Ethereum(ethereum_address)
		}
		Proof::RelayChain(relay_account, relay_proof) => {
			if verify_relay(
				&prefix,
				associate_origin.reward_account,
				relay_account.clone(),
				&relay_proof,
			) {
				RemoteAccount::RelayChain(relay_account)
			} else {
				return Ok(StatusCode::BAD_REQUEST);
			}
		}
	};

	if contributors.shares.contains_key(&remote_account) {
		api.tx(); // not yet implemented
	}

	Ok(StatusCode::OK)
}
