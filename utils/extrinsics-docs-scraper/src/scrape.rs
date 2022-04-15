use heck::{ToSnakeCase, ToTitleCase};
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};
use std::{
	fmt::{self, Write},
	fs, io,
	path::PathBuf,
};
use syn::{parse::Parser as _, Attribute, Lit, Meta, MetaNameValue};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::pallet_info::PalletInfo;

/// Generates the docs for a pallet given the path to the lib.rs file and the destination file to
/// write the documentation to.
///
/// # Errors
///
/// See [`ScrapeError`] for all possible errors.
pub(crate) fn generate_docs(
	pallet_info: &PalletInfo,
	docs_root_url: &str,
) -> Result<(), ScrapeError> {
	log::debug!("reading input file \"{}\"", &pallet_info.lib_rs_path.to_string_lossy());
	let input_file = fs::read_to_string(&pallet_info.lib_rs_path)?;

	// pallet module will be annotated with this macro
	let frame_support_pallet_attr =
		Attribute::parse_outer.parse_str("#[frame_support::pallet]").unwrap().remove(0);

	// extrinsics impl will be annotated with this attribute
	let pallet_call_attr = Attribute::parse_outer.parse_str("#[pallet::call]").unwrap().remove(0);

	let mut extrinsics_docs_out_string = String::new();

	let extrinsics_definitions = syn::parse_file(&input_file)
		.map_err(|_| ScrapeError::Parse(pallet_info.lib_rs_path.clone()))?
		.items // all items in file
		.into_iter()
		.find_map(|top_level_item| match top_level_item {
			// check if the #[frame_support::pallet] attribute is one of the attributes on the
			// module
			//
			// i'm unsure if there would ever be a case where the pallet module would be
			// annotated with anything else? perhaps an #[allow(..)] or something similar. same
			// logic applies to finding the extrinsics impl.
			syn::Item::Mod(module) if module.attrs.contains(&frame_support_pallet_attr) =>
				Some(module),
			_ => None,
		}) // find the module in the file that has the attribute
		.ok_or_else(|| ScrapeError::PalletAttributeNotPresent(pallet_info.lib_rs_path.clone()))?
		.content // said module's contents
		.ok_or_else(|| ScrapeError::PalletModuleEmpty(pallet_info.lib_rs_path.clone()))?
		.1 // the actual items in the module (.0 is just the opening brace)
		.into_iter()
		.find_map(|pallet_module_item| match pallet_module_item {
			// check if the #[pallet::call] attribute is one of the attributes on the module
			syn::Item::Impl(imp) if imp.attrs.contains(&pallet_call_attr) => Some(imp),
			_ => None,
		}) // find the impl block with the pallet::call attribute
		.ok_or_else(|| ScrapeError::PalletCallAttributeNotPresent(pallet_info.lib_rs_path.clone()))?
		.items // the items in said impl block
		.into_iter()
		.filter_map(|outer_mod_item| match outer_mod_item {
			// all the items in the pallet::call section should be a method in a working pallet
			// we've gotten this far, just assume that the extrinsics are correct
			syn::ImplItem::Method(method) => Some(method),
			_ => None,
		}); // only the extrinsics

	// automatically generated information
	writeln!(&mut extrinsics_docs_out_string, "<!-- AUTOMATICALLY GENERATED -->",)?;
	writeln!(
		&mut extrinsics_docs_out_string,
		"<!-- Generated at {} -->",
		OffsetDateTime::now_utc().format(&Rfc3339).unwrap()
	)?;
	writeln!(&mut extrinsics_docs_out_string)?;

	// pallet name header
	writeln!(
		&mut extrinsics_docs_out_string,
		"# {} Pallet Extrinsics",
		pallet_info.pallet_name.to_title_case()
	)?;

	for extrinsic_definition in extrinsics_definitions {
		let extrinsic_name = extrinsic_definition.sig.ident.to_string();

		log::debug!("writing header for extrinsic `{}`", extrinsic_name);

		// extrinsic header
		writeln!(&mut extrinsics_docs_out_string)?;
		writeln!(&mut extrinsics_docs_out_string, "## {}", extrinsic_name.to_title_case())?;

		writeln!(&mut extrinsics_docs_out_string)?;
		writeln!(
			&mut extrinsics_docs_out_string,
			"[`{extrinsic_name}`]({docs_root_url}{pallet_name_full}/pallet/enum.Call.html#variant.{extrinsic_name})",
			extrinsic_name = &extrinsic_name,
			pallet_name_full = &pallet_info.pallet_name_full.to_snake_case(),
			docs_root_url = docs_root_url,
		)?;

		match get_docs_from_attrs(&extrinsic_name, &pallet_info, &extrinsic_definition.attrs)? {
			Some(s) => {
				let final_docs = alter_markdown(&s, &extrinsic_name, pallet_info);
				// extrinsic documentation
				writeln!(&mut extrinsics_docs_out_string)?;
				writeln!(&mut extrinsics_docs_out_string, "{}", final_docs)?;

				log::info!("Wrote documentation for extrinsic `{}`", extrinsic_name);
			},
			None => {
				writeln!(&mut extrinsics_docs_out_string)?;
				writeln!(
					&mut extrinsics_docs_out_string,
					"No documentation available at this time."
				)?;

				log::warn!(
					"No documentation found for extrinsic `{}` in pallet `{}`",
					&extrinsic_name,
					&pallet_info.pallet_name
				);
			},
		}
	}

	let extrinsics_output_file_path = pallet_info.docs_output_paths.extrinsics_docs_file_path();
	log::debug!("opening output file \"{}\"", extrinsics_output_file_path.to_string_lossy());
	let mut out_file = fs::OpenOptions::new()
		.truncate(true)
		.create(true)
		.write(true)
		.open(extrinsics_output_file_path)?;

	io::Write::write_all(&mut out_file, extrinsics_docs_out_string.as_bytes())?;

	Ok(())
}

/// Performs various alterations to the provided markdown input.
///
/// Changes made:
///
/// - All headings are increased in depth by 2 levels.
fn alter_markdown(input: &str, extrinsic_name: &str, pallet_info: &PalletInfo) -> String {
	let mut bad_heading = false;
	let parser = Parser::new(input);
	let mut altered_events =
		Vec::with_capacity(parser.size_hint().1.unwrap_or(parser.size_hint().0));

	for event in parser {
		match event {
			Event::Start(Tag::Heading(level, ident, classes)) =>
				altered_events.push(Event::Start(Tag::Heading(
					match HeadingLevel::try_from((level as usize) + 2_usize) {
						Ok(ok) => ok,
						Err(_) => {
							bad_heading = true;
							HeadingLevel::H6
						},
					},
					ident,
					classes,
				))),
			Event::End(Tag::Heading(level, ident, classes)) =>
				altered_events.push(Event::End(Tag::Heading(
					HeadingLevel::try_from((level as usize) + 2_usize).unwrap_or(HeadingLevel::H6),
					ident,
					classes,
				))),
			Event::Text(text) => {
				if bad_heading {
					log::error!(
						"Heading `{}` in extrinsic `{}` for pallet `{}` is nested too deep and will be defaulted to the maximum depth (6). All headings are increased in depth by 2 for the mdbook documentation, ensure that the maximum heading depth in the doc comments is 4 (####).",
						&text,
						extrinsic_name,
						pallet_info.pallet_name
					)
				}
				bad_heading = false;
				altered_events.push(Event::Text(text));
			},
			e => altered_events.push(e),
		}
	}

	let mut out_string = String::new();

	pulldown_cmark_to_cmark::cmark(altered_events.iter(), &mut out_string)
		.unwrap()
		.finalize(&mut out_string)
		.unwrap();

	out_string
}

/// Parses the doc comments out of the attributes on an item.
///
/// When doc comments are desugared, they still contain the leading space (between the /// and the
/// content):
///
/// ```rust,ignore
/// //       ↓ leading space here
/// #[doc = " doc here"]
/// fn cool_function() {}
/// ```
///
/// Since we (typically) use /// style for doc comments, unconditionally remove the leading space
/// and error otherwise.
///
/// If there are no docs on
///
/// # Errors
///
/// Returns an error if the any of the doc comment lines (excluding blank lines) don't have a
/// leading space.
pub(crate) fn get_docs_from_attrs(
	item_name: &str,
	pallet_info: &PalletInfo,
	attrs: &[Attribute],
) -> anyhow::Result<Option<String>, ScrapeError> {
	let doc_lines = attrs
		.iter()
		.filter_map(|attr| {
			// pull the doc attributes out of the attributes
			match attr.parse_meta() {
				Ok(Meta::NameValue(MetaNameValue {
					path,
					eq_token: _,
					lit: Lit::Str(doc_str),
				})) if path.is_ident("doc") => Some(doc_str.value()),
				_ => None,
			}
		})
		.map(|doc_string| {
			// blank lines won't have a leading space
			if doc_string.is_empty() {
				Ok(Some(doc_string))
			} else {
				doc_string
					.strip_prefix(' ')
					.map(ToOwned::to_owned)
					.ok_or(ScrapeError::ImproperlyFormattedDocs {
						// currently this is only used for extrinsics, will require some refactoring
						// to get it to work with any item
						extrinsic_name: item_name.to_string(),
						pallet_info: pallet_info.clone(),
					})
					.map(Some)
			}
		})
		.filter_map(Result::transpose)
		.collect::<Result<Vec<_>, ScrapeError>>()?;

	Ok((!doc_lines.is_empty()).then(move || doc_lines.join("\n")))
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ScrapeError {
	#[error("IO Error")]
	Io(#[from] io::Error),
	#[error("Error writing documentation")]
	Fmt(#[from] fmt::Error),
	#[error(
		"Unable to parse file \"{}\" as rust source",
		.0.to_string_lossy()
	)]
	Parse(PathBuf),
	#[error(
		"No #[frame_support::pallet] attribute present in file. Is the file \"{}\" a pallet?",
		.0.to_string_lossy()
	)]
	PalletAttributeNotPresent(PathBuf),
	#[error(
		"Module annotated by #[frame_support::pallet] has no content. Is the file \"{}\" a pallet?",
		.0.to_string_lossy()
	)]
	PalletModuleEmpty(PathBuf),
	#[error(
		"No #[pallet::call] attribute present in file  \"{}\". Are there extrinsics for this pallet?",
		.0.to_string_lossy()
	)]
	PalletCallAttributeNotPresent(PathBuf),
	#[error(
		"Unable to remove leading space for doc comment on extrinsic `{}` in pallet `{}`. Ensure that the doc comment uses /// style.",
		.extrinsic_name,
		.pallet_info.pallet_name
	)]
	ImproperlyFormattedDocs { extrinsic_name: String, pallet_info: PalletInfo },
}
