use std::{fs, io::Write, path::PathBuf};

use anyhow::Context;
use clap::{Arg, Command};
use env_logger::fmt::Color;
use log::LevelFilter;
use syn::{parse::Parser, Attribute, Lit, Meta, MetaNameValue};

const PATH_ARG: &str = "PATH";
const OUTPUT_PATH_ARG: &str = "OUTPUT_PATH";
const VERBOSITY_ARG: &str = "VERBOSITY";

fn main() -> anyhow::Result<()> {
	let matches = Command::new("extrinsics-docs-scraper")
		.arg_required_else_help(true)
		.arg(
			Arg::new(PATH_ARG)
				.long("path")
				.takes_value(true)
				.forbid_empty_values(true)
				.required(true)
				.help("Path to input file. Should be the file containing the pallet's `frame_support::pallet` macro.")
		).arg(
			Arg::new(OUTPUT_PATH_ARG)
				.long("output-path")
				.takes_value(true)
				.forbid_empty_values(true)
				.required(true)
				.help("The output path for the generated documentation.")
		).arg(
			Arg::new(VERBOSITY_ARG)
				.long("verbose")
				.short('v')
				.takes_value(false)
				.multiple_occurrences(true)
				.max_occurrences(3)
				.help("Verbosity for output logging. Can be specified multiple times, up to 3 total.")
		).get_matches();

	let verbosity = matches.occurrences_of(VERBOSITY_ARG);
	let path = matches.value_of_t(PATH_ARG).unwrap_or_else(|e| e.exit());
	let output_path = matches.value_of_t(OUTPUT_PATH_ARG).unwrap_or_else(|e| e.exit());

	init_logger(verbosity);

	generate_docs(path, output_path)?;

	Ok(())
}

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
			log::Level::Info => writeln!(f, "{}", record.args()),
			log::Level::Debug => writeln!(f, "{}", record.args()),
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

fn generate_docs(path: PathBuf, output_path: PathBuf) -> anyhow::Result<()> {
	let input_file = fs::read_to_string(path).context("Unable to read input file")?;

	let ast = syn::parse_file(&input_file).unwrap();

	// pallet module will be annotated with this macro
	let frame_support_pallet_attr =
		Attribute::parse_outer.parse_str("#[frame_support::pallet]").unwrap();

	// extrinsics impl will be annotated with this attribute
	let pallet_call_attr = Attribute::parse_outer.parse_str("#[pallet::call]").unwrap();

	let mut out_file = fs::OpenOptions::new()
		.create(true)
		.write(true)
		.open(output_path)
		.context("Unable to open output file")?;

	let item_mod = ast
		.items
		.into_iter()
		.find_map(|outer_mod_item| match outer_mod_item {
			syn::Item::Mod(module) if module.attrs == frame_support_pallet_attr => Some(module),
			_ => None,
		})
		.context("No #[frame_support::pallet] attribute present in file; is this file a pallet?")?;

	let imp = item_mod
		.content
		.context(
			"Module annotated by #[frame_support::pallet] has no content; is this file a pallet?",
		)?
		.1
		.into_iter()
		.find_map(|inner_mod_item| match inner_mod_item {
			syn::Item::Impl(imp) if imp.attrs == pallet_call_attr => Some(imp),
			_ => None,
		})
		.context(
			"No #[pallet::call] attribute present in file; are there extrinsics for this pallet?",
		)?;

	for impl_item in imp.items {
		if let syn::ImplItem::Method(method) = impl_item {
			let extrinsic_name = method.sig.ident.to_string();

			// extrinsic header
			writeln!(&mut out_file, "")?;
			writeln!(&mut out_file, "## {}", extrinsic_name)?;

			match get_docs_from_attrs(&extrinsic_name, method.attrs)? {
				Some(s) => {
					// extrinsic documentation
					writeln!(&mut out_file, "")?;
					writeln!(&mut out_file, "{}", s)?;

					log::info!("Wrote documentation for extrinsic {}", extrinsic_name);
				},
				None => {
					log::warn!("No documentation found for extrinsic {}", &extrinsic_name);
				},
			}
		} else {
			continue;
		}
	}

	Ok(())
}

fn get_docs_from_attrs(name: &str, attrs: Vec<Attribute>) -> anyhow::Result<Option<String>> {
	let doc_lines = attrs
		.iter()
		.filter_map(|attr| {
			if let Ok(Meta::NameValue(MetaNameValue {
				path,
				eq_token: _,
				lit: Lit::Str(lit_str),
			})) = attr.parse_meta()
			{
				if path.is_ident("doc") {
					// when doc comments are desugared, they still contain the leading space (between the /// and the content):
					//
					// #[doc = " doc here"]
					//          ^ leading space here
					//
					// Since we (typically) use /// style for doc comments, unconditionally remove the leading space and error otherwise.

					let doc_string = lit_str.value();

					// blank lines won't have a leading space
					if doc_string.is_empty() {
						return Some(Ok(doc_string));
					}

					match doc_string
							.strip_prefix(' ')
							.map(ToOwned::to_owned)
							.context(format!("Unable to remove leading space for doc comment on extrinsic `{}`. Ensure that the doccomment uses /// style.", name)) {
						Ok(cleaned_doc_string) => Some(anyhow::Result::Ok(cleaned_doc_string.to_owned())),
						Err(why) => return Some(Err(why)),
					}
				} else {
					None
				}
			} else {
				None
			}
		})
		.collect::<anyhow::Result<Vec<_>>>()?;

	Ok((!doc_lines.is_empty()).then(move || doc_lines.join("\n")))
}
