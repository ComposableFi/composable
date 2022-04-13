use std::{fs, path::PathBuf};

use anyhow::Context;
use clap::{Arg, Command};
use syn::{
	parse::Parser,
	punctuated::Punctuated,
	token::{Colon2, Pound},
	Attribute, Ident, Lit, LitStr, Meta, MetaNameValue, Path, PathSegment, Token,
};

const PATH_ARG: &str = "PATH";
const OUTPUT_PATH_ARG: &str = "OUTPUT_PATH";

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
		).get_matches();

	let path = matches.value_of_t(PATH_ARG).unwrap_or_else(|e| e.exit());
	let output_path = matches.value_of_t(OUTPUT_PATH_ARG).unwrap_or_else(|e| e.exit());
	generate_docs(path, output_path)?;

	Ok(())
}

fn generate_docs(path: PathBuf, output_path: PathBuf) -> anyhow::Result<()> {
	let input_file = fs::read_to_string(path).context("unable to read file")?;

	let ast = syn::parse_file(&input_file).unwrap();

	let frame_support_pallet_attr =
		Attribute::parse_outer.parse_str("#[frame_support::pallet]").unwrap();
	let pallet_call_attr = Attribute::parse_outer.parse_str("#[pallet::call]").unwrap();
	for outer_mod_item in ast.items {
		match outer_mod_item {
			syn::Item::Mod(module) => {
				if module.attrs == frame_support_pallet_attr {
					println!("found pallet macro!");

					for inner_mod_item in module.content.unwrap().1 {
						match inner_mod_item {
							syn::Item::Impl(imp) => {
								if imp.attrs == pallet_call_attr {
									println!("found pallet call attribute!");
									for impl_item in imp.items {
										match impl_item {
											syn::ImplItem::Method(method) => {
												println!();
												println!("## {}", method.sig.ident.to_string());
												println!();
												println!("{}", get_docs_from_attrs(method.attrs));
											},
											_ => todo!(),
										}
									}
								}
							},
							_ => continue,
						}
					}
				}
			},
			_ => continue,
		}
	}

	todo!()
}

fn get_docs_from_attrs(attrs: Vec<Attribute>) -> String {
	attrs
		.iter()
		.filter_map(|attr| {
			if let Ok(Meta::NameValue(MetaNameValue {
				path,
				eq_token: _,
				lit: Lit::Str(lit_str),
			})) = attr.parse_meta()
			{
				if path.is_ident("doc") {
					Some(lit_str.value())
				} else {
					None
				}
			} else {
				None
			}
		})
		.collect::<Vec<_>>()
		.join("\n")
}
