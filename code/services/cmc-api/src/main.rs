use axum::{http::StatusCode, routing::get, Router};
use clap::Parser;
use std::net::SocketAddr;

#[derive(clap::Parser)]
pub struct Cli {
	/// The address to bind the server to.
	address: SocketAddr,
}

#[tokio::main]
async fn main() {
	// initialize tracing
	tracing_subscriber::fmt::init();

	let cli = Cli::parse();

	// build our application with a route
	let app = build_router();

	// run our app with hyper
	// `axum::Server` is a re-export of `hyper::Server`
	let addr = cli.address;
	tracing::info!("listening on {}", addr);
	axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn healthcheck() -> &'static str {
	"Hello, World!"
}

const BILLION: u128 = 1_000_000_000_000;

#[axum_macros::debug_handler]
async fn total_supply() -> Result<String, StatusCode> {
	Ok((BILLION * 10).to_string())
}

#[axum_macros::debug_handler]
async fn circulating_supply() -> Result<String, StatusCode> {
	Ok((BILLION + BILLION / 2).to_string())
}

#[axum_macros::debug_handler]
async fn explorer_url() -> Result<String, StatusCode> {
	Ok(String::from("https://picasso.subscan.io/"))
}

#[axum_macros::debug_handler]
async fn rich_list_url() -> Result<String, StatusCode> {
	Ok(String::from(""))
}

pub fn build_router() -> Router {
	Router::new()
		.route("/healthcheck", get(healthcheck))
		.route("/total_supply", get(total_supply))
		.route("/circulating_supply", get(circulating_supply))
		.route("/explorer_url", get(explorer_url))
		.route("/rich_list_url", get(rich_list_url))
}
