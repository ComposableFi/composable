use anyhow::Context;
use std::{fs, io::Write, path::Path};
use syn::{parse::Parser, Attribute, Lit, Meta, MetaNameValue};

/// Generates the docs for a pallet given the path to the lib.rs file and the destination file to
/// write the documentation to.
///
/// Currently this only extracts the extrinsics, and as such `output_path` is expected to be the
/// path directly to the `extrinsics.md` in the pallet's corresponding section in the book (i.e.
/// `book/stc/pallets/<pallet-name>/extrinsics.md`).
///
/// # Errors
///
/// Will error if the file cannot be parsed as a pallet, if the pallet doesn't have any extrinsics,
/// or if there are any issues reading/ writing the files at the provided paths.
pub fn generate_docs(path: &Path, output_path: &Path) -> anyhow::Result<()> {
	let input_file = fs::read_to_string(path).context("Unable to read input file")?;

	let ast =
		syn::parse_file(&input_file).context("Unable to parse provided file as rust source")?;

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
		.context(format!(
			"No #[frame_support::pallet] attribute present in file \"{}\"; is this file a pallet?",
			&path.to_string_lossy()
		))?;

	let imp = item_mod
		.content
		.context(format!(
			"Module annotated by #[frame_support::pallet] has no content; is the file \"{}\" a pallet?",
			&path.to_string_lossy()
		))?
		.1
		.into_iter()
		.find_map(|inner_mod_item| match inner_mod_item {
			syn::Item::Impl(imp) if imp.attrs == pallet_call_attr => Some(imp),
			_ => None,
		})
		.context(format!(
			"No #[pallet::call] attribute present in file  \"{}\"; are there extrinsics for this pallet?",
			&path.to_string_lossy()
		))?;

	for impl_item in imp.items {
		if let syn::ImplItem::Method(method) = impl_item {
			let extrinsic_name = method.sig.ident.to_string();

			// extrinsic header
			writeln!(&mut out_file)?;
			writeln!(&mut out_file, "## {}", extrinsic_name)?;

			match get_docs_from_attrs(&extrinsic_name, &method.attrs)? {
				Some(s) => {
					// extrinsic documentation
					writeln!(&mut out_file)?;
					writeln!(&mut out_file, "{}", s)?;

					log::info!("Wrote documentation for extrinsic {}", extrinsic_name);
				},
				None => {
					log::warn!("No documentation found for extrinsic {}", &extrinsic_name);
				},
			}
		}
	}

	Ok(())
}

/// Parses the doc comments out of the attributes on an item
/// when doc comments are desugared, they still contain the leading space (between the /// and the
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
/// # Errors
///
/// Returns an error if the doc comment doesn't have a leading space.
pub fn get_docs_from_attrs(name: &str, attrs: &[Attribute]) -> anyhow::Result<Option<String>> {
	let doc_lines = attrs
		.iter()
		.map(|attr| {
			if let Ok(Meta::NameValue(MetaNameValue {
				path,
				eq_token: _,
				lit: Lit::Str(lit_str),
			})) = attr.parse_meta()
			{
				if path.is_ident("doc") {
					let doc_string = lit_str.value();

					// blank lines won't have a leading space
					if doc_string.is_empty() {
						return Ok(Some(doc_string));
					}

					doc_string
							.strip_prefix(' ')
							.map(ToOwned::to_owned).map(Some)
							.context(format!("Unable to remove leading space for doc comment on extrinsic `{}`. Ensure that the doc comment uses /// style.", name))
				} else {
					Ok(None)
				}
			} else {
				Ok(None)
			}
		})
		.filter_map(Result::transpose)
		.collect::<anyhow::Result<Vec<_>>>()?;

	Ok((!doc_lines.is_empty()).then(move || doc_lines.join("\n")))
}
