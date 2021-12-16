/// this must be singleton
#[cfg(test)]
pub fn env_logger_init() {
	let _ = env_logger::builder().is_test(true).try_init();
}

#[cfg(test)]
mod kusama_test_net;

#[cfg(test)]
mod xcm_tests;

#[cfg(test)]
mod cross_chain_transfer;

#[cfg(test)]
mod runtime_tests;
