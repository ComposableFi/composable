use std::{collections::HashMap, fs, ops::Not, path::PathBuf, sync::mpsc::channel, time::Duration};

use anyhow::Context;
use clap::Parser;
use log::LevelFilter;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};

mod pallet_info;
mod scrape;

use crate::{pallet_info::get_pallet_info, scrape::generate_docs};

#[derive(Parser)]
#[clap(arg_required_else_help(true))]
struct Cli {
	/// The path to the configuration file.
	#[clap(
		long,
		parse(from_os_str),
		value_hint(clap::ValueHint::AnyPath),
		forbid_empty_values(true)
	)]
	config_file_path: PathBuf,

	/// Set the verbosity for output logging. Can be specified up to 3 times for more verbose
	/// output.
	///
	/// Warnings and errors are emitted by default. Errors will always be emitted.
	#[clap(
		short,
		long = "verbose",
		takes_value(false),
		multiple_occurrences(true),
		max_occurrences(3),
		parse(from_occurrences),
		verbatim_doc_comment
	)]
	verbosity: u8,

	/// Enable hot-reloading of the documentation by watching for file changes in the provided
	/// frame_directory_path.
	#[clap(long, parse(from_flag))]
	watch: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
	/// The path to the FRAME directory containing all the pallets to gather documentation from.
	frame_directory_path: PathBuf,

	/// The URL to the root of the rust docs.
	///
	/// Ensure that the URL passed includes the path to the docs, not just the root url of the
	/// hosting site (i.e. if the docs are hosted at /doc/, supply https://some.site.com/doc/ instead
	/// of https://some.site.com/). The generated documentation will use this as the base to link to
	/// the rust docs with, appending directly to the end of the url. A trailing `/` is optional,
	/// and will be appended if not provided.
	// TODO: Use an actual URL/URI type for this
	docs_root_url: String,

	/// The output path for the generated documentation. Files will be written to
	/// <output-path>/<pallet>/.
	///
	/// The provided directory will be scanned for sub-directories that match the pallets found in
	/// <FRAME_DIRECTORY_PATH>. Any pallets that don't have their own subfolder will be skipped and
	/// a warning will be emitted.
	output_path: PathBuf,

	/// A list of folders in the provided FRAME directory that should be ignored.
	exclude: Vec<String>,
}

fn main() -> anyhow::Result<()> {
	let args = Cli::parse();

	init_logger(args.verbosity);

	let config: Config = toml::from_str(
		&fs::read_to_string(args.config_file_path)
			.context("Unable to read file at provided config-file-path")?,
	)
	.context("Unable to parse configuration file as TOML")?;

	let docs_root_url_cleaned = config
		.docs_root_url
		.ends_with('/')
		.not()
		.then(|| String::from_iter([&config.docs_root_url, "/"]))
		.unwrap_or_else(|| config.docs_root_url);

	let pallet_infos = get_pallet_info(
		&config
			.frame_directory_path
			.canonicalize()
			.context("Unable to canonicalize input directory")?,
		&config.output_path,
		&config.exclude,
	)?;

	let pallet_lib_rs_path_to_pallet_info_map = pallet_infos
		.into_iter()
		.map(|entry| (entry.lib_rs_path.clone(), entry))
		.collect::<HashMap<_, _>>();

	// generate docs for all pallets
	for (_, pallet_info) in pallet_lib_rs_path_to_pallet_info_map.iter() {
		if let Err(why) = generate_docs(&pallet_info, &docs_root_url_cleaned) {
			log::error!("{}", why);
		}
	}

	if args.watch {
		// wait a few seconds to allow the previous files to save
		std::thread::sleep(Duration::from_nanos(2));

		// Create a channel to receive the events.
		let (tx, rx) = channel();

		// Create a watcher object, delivering debounced events.
		// The notification back-end is selected based on the platform.
		let mut watcher = watcher(tx, Duration::from_secs(2)).unwrap();

		for (_, pallet_info) in &pallet_lib_rs_path_to_pallet_info_map {
			watcher.watch(&pallet_info.lib_rs_path, RecursiveMode::NonRecursive).unwrap();
		}

		loop {
			match rx.recv() {
				Ok(event) => {
					match event {
						// Only write events matter, since the lib.rs files shouldn't be
						// moved, renamed or deleted (write events will always follow create
						// events). If pallets are being refactored, hot-reloading the book is most
						// likely not a priority.
						DebouncedEvent::Write(changed_file_path) => {
							let pallet_info = pallet_lib_rs_path_to_pallet_info_map
								.get(&*changed_file_path)
								.with_context(|| {
									format!(
										"received event about non-watched file \"{}\"",
										&changed_file_path.to_string_lossy()
									)
								})?;

							log::info!(
								"Changed file detected, regenerating documentation for `{}`",
								pallet_info.pallet_name
							);

							if let Err(why) = generate_docs(&pallet_info, &docs_root_url_cleaned) {
								log::error!("{}", why);
							}
						},
						_ => {},
					}
				},
				Err(e) => log::error!("Error watching files: {:?}", e),
			}
		}
	}

	Ok(())
}

/// Initializes the logger for the crate with the given verbosity.
fn init_logger(verbosity: u8) {
	let mut logger = env_logger::Builder::new();
	logger.format_module_path(false);
	logger.format_target(false);
	logger.format_timestamp(None);
	match verbosity {
		0 => logger.filter_level(LevelFilter::Warn),
		1 => logger.filter_level(LevelFilter::Info),
		2 => logger.filter_level(LevelFilter::Debug),
		3 => logger.filter_level(LevelFilter::Trace),
		_ => unreachable!(),
	};
	logger.init();
}
