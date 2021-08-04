use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "1.0", author = "MLabs")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
  #[clap(short, long, default_value = "http://127.0.0.1:8910")]
  pub pythd_host: String,
  #[clap(short, long, default_value = "127.0.0.1:8081")]
  pub listening_address: String
}

pub fn get_opts() -> Opts {
  Opts::parse()
}
