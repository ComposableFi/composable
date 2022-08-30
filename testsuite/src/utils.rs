use std::{future::Future, time::Duration};

pub fn parse_amount(amount: String) -> u128 {
	str::parse::<u128>(&amount).expect("Failed to parse as u128")
}

pub async fn timeout_future<T: Future>(future: T, secs: u64, reason: String) -> T::Output {
	let duration = Duration::from_secs(secs);
	match tokio::time::timeout(duration.clone(), future).await {
		Ok(output) => output,
		Err(_) => panic!("Future didn't finish within {duration:?}, {reason}"),
	}
}
