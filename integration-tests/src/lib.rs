/// this must be singleton
#[cfg(test)]
pub fn env_logger_init() {
	use std::sync::Once;
	static LOG_INIT: Once = Once::new();
	LOG_INIT.call_once(|| {
		env_logger::init();
	});
}

#[cfg(test)]
mod kusama_test_net;

#[cfg(test)]
mod xcm_tests;

#[cfg(test)]
mod runtime_tests;
