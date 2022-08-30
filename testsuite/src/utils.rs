use std::{future::Future, time::Duration};

pub fn parse_amount(amount: String) -> u128 {
	str::parse::<u128>(&amount).expect("Failed to parse as u128")
}

pub async fn timeout_future<T: Future>(future: T, secs: u64, reason: String) {
	let duration = Duration::from_secs(secs);
	if let Err(_) = tokio::time::timeout(duration.clone(), future).await {
		panic!("Future didn't finish within {duration:?}, {reason}")
	}
}
