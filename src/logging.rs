use log::LevelFilter;

pub fn setup_logging()  {
	env_logger::builder().filter_module("hyperspace", LevelFilter::Info).init();
}
