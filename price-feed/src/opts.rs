use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "MLabs")]
pub struct Opts {
	#[clap(short, long, default_value = "http://127.0.0.1:8910")]
	pub pythd_host: String,
	#[clap(short, long, default_value = "127.0.0.1:3001")]
	pub listening_address: String,
}

pub fn get_opts() -> Opts {
	Opts::parse()
}
