use std::{
	collections::HashMap,
	fs::{self, DirEntry},
	io::{self, Write},
	path::{Path, PathBuf},
	sync::mpsc::channel,
	time::Duration,
};

use anyhow::Context;
use clap::{Arg, Command};
use env_logger::fmt::Color;
use extrinsics_docs_scraper::generate_docs;
use log::LevelFilter;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};

const FRAME_DIR_PATH_ARG: &str = "FRAME_DIR_PATH";
const BOOK_PALLETS_DOCS_OUTPUT_PATH_ARG: &str = "BOOK_PALLETS_DOCS_OUTPUT_PATH";
const VERBOSITY_ARG: &str = "VERBOSITY";

fn main() -> anyhow::Result<()> {
	let matches = Command::new("extrinsics-docs-scraper")
		.arg_required_else_help(true)
		.arg(
			Arg::new(FRAME_DIR_PATH_ARG)
				.long("path")
				.takes_value(true)
				.value_hint(clap::ValueHint::AnyPath)
				.forbid_empty_values(true)
				.required(true)
				.help("Path to the FRAME directory containing all the pallets to gather documentation from.")
		).arg(
			Arg::new(BOOK_PALLETS_DOCS_OUTPUT_PATH_ARG)
				.long("output-path")
				.takes_value(true)
				.value_hint(clap::ValueHint::AnyPath)
				.forbid_empty_values(true)
				.required(true)
				.help("The output path for the generated documentation. Files will be written to <output-path>/<pallet>/.")
		).arg(
			Arg::new(VERBOSITY_ARG)
				.long("verbose")
				.short('v')
				.takes_value(false)
				.multiple_occurrences(true)
				.max_occurrences(3)
				.help("Verbosity for output logging. Can be specified multiple times, up to 3 times total.")
		).get_matches();

	let verbosity = matches.occurrences_of(VERBOSITY_ARG);
	let path: PathBuf = matches.value_of_t(FRAME_DIR_PATH_ARG).unwrap_or_else(|e| e.exit());
	let output_path: PathBuf = matches
		.value_of_t(BOOK_PALLETS_DOCS_OUTPUT_PATH_ARG)
		.unwrap_or_else(|e| e.exit());

	init_logger(verbosity);

	// Create a channel to receive the events.
	let (tx, rx) = channel();

	// Create a watcher object, delivering debounced events.
	// The notification back-end is selected based on the platform.
	let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

	let pallet_entries = get_pallet_info(&path, &output_path)?;

	for pallet_entry in &pallet_entries {
		watcher.watch(&pallet_entry.lib_rs_path, RecursiveMode::NonRecursive).unwrap();
	}

	let path_map = pallet_entries
		.into_iter()
		.map(|entry| (entry.lib_rs_path.clone(), entry))
		.collect::<HashMap<_, _>>();

	loop {
		match rx.recv() {
			Ok(event) => {
				match event {
					// Only write & create events matter, since the lib.rs files shouldn't be moved, renamed or deleted.
					// If pallets are being refactored, hot-reloading the book is most likely not a priority
					DebouncedEvent::NoticeWrite(changed_file_path)
					| DebouncedEvent::Create(changed_file_path)
					| DebouncedEvent::Write(changed_file_path) => {
						let pallet_info = path_map.get(&*changed_file_path).with_context(|| {
							format!(
								"recieved event about non-watched file \"{}\"",
								&changed_file_path.to_string_lossy()
							)
						})?;

						log::info!(
							"Changed file detected, regenerating documentation for {}",
							pallet_info.name
						);

						if let Err(why) =
							generate_docs(&changed_file_path, &*pallet_info.docs_output_path)
						{
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

fn get_pallet_info(path: &Path, output_path: &Path) -> Result<Vec<PalletInfo>, anyhow::Error> {
	let pallet_entries = fs::read_dir(&path)
		.context("Unable to read input directory")?
		.map(|dir_entry: io::Result<DirEntry>| -> anyhow::Result<Option<PalletInfo>> {
			let dir_entry = dir_entry?;

			if let Ok(metadata) = dir_entry.metadata() {
				if metadata.is_dir() {
					let name = dir_entry
						.path()
						.file_name()
						.unwrap()
						.to_str()
						.map(ToOwned::to_owned)
						.context("File path was not valid unicode")?;

					let mut lib_rs_path = dir_entry.path();
					lib_rs_path.extend(["src", "lib.rs"]);
					lib_rs_path
						.exists()
						.then(|| ())
						.context(format!("Pallet {} does not have a lib.rs file", &name))?;

					let mut docs_output_path = output_path.to_owned();
					docs_output_path.extend([&name]);
					if docs_output_path.exists() {
						docs_output_path.extend(["extrinsics.md"]);
						Ok(Some(PalletInfo { name, lib_rs_path, docs_output_path }))
					} else {
						log::warn!(
							"Pallet {} does not have a section in the book; skipping.",
							&name
						);
						Ok(None)
					}
				} else {
					Ok(None)
				}
			} else {
				Ok(None)
			}
		})
		.filter_map(Result::transpose)
		.collect::<anyhow::Result<Vec<PalletInfo>>>()?;
	Ok(pallet_entries)
}

/// Initializes the logger for the crate with the given verbosity.
fn init_logger(verbosity: u64) {
	let mut logger = env_logger::Builder::new();
	logger.format(|f, record| {
		let mut style = f.default_level_style(record.level());
		match record.level() {
			log::Level::Error => {
				writeln!(f, "{}", style.value(format!("{}: {}", record.level(), record.args())))
			},
			log::Level::Warn => {
				writeln!(f, "{}: {}", style.set_bold(true).value("warning"), record.args())
			},
			log::Level::Info | log::Level::Debug => writeln!(f, "{}", record.args()),
			log::Level::Trace => {
				writeln!(f, "{}", f.style().set_color(Color::Black).value(record.args()))
			},
		}
	});
	match verbosity {
		0 => logger.filter_level(LevelFilter::Warn),
		1 => logger.filter_level(LevelFilter::Info),
		2 => logger.filter_level(LevelFilter::Debug),
		3 => logger.filter_level(LevelFilter::Trace),
		_ => unreachable!(),
	};
	logger.init();
}

struct PalletInfo {
	#[allow(dead_code)] // will be used in the future
	name: String,
	lib_rs_path: PathBuf,
	docs_output_path: PathBuf,
}
