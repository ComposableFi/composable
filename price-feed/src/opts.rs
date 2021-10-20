// use clap::{AppSettings, Clap};

// #[derive(Clap)]
#[derive(Default)]
// #[clap(version = "1.0", author = "MLabs")]
// #[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
	//#[clap(short, long, default_value = "http://127.0.0.1:8910")]
	pub pythd_host: String,
	//#[clap(short, long, default_value = "127.0.0.1:3001")]
	pub listening_address: String,
}

impl Opts {
	pub fn new () -> Self {
		Opts::default()
	}
}

// pub fn get_opts() -> Opts {
// 	Opts::parse()
// }
