use anyhow::Context;
use std::{
	fs::{self, DirEntry},
	io,
	path::{Path, PathBuf},
};

/// Reads in the pallets in the provided FRAME directory and gets information about all of the
/// pallets within it.
// REVIEW: Maybe read the workspace Cargo.toml instead?
pub(crate) fn get_pallet_info(
	frame_dir_path: &Path,
	pallet_docs_output_path: &Path,
) -> Result<Vec<PalletInfo>, anyhow::Error> {
	let pallet_entries = fs::read_dir(&frame_dir_path)
		.context("Unable to read input directory")?
		.map(|dir_entry: io::Result<DirEntry>| -> anyhow::Result<Option<PalletInfo>> {
			let dir_entry = dir_entry.context("Error reading input directory")?;

			if let Ok(metadata) = dir_entry.metadata() {
				if metadata.is_dir() {
					let pallet_name = dir_entry
						.path()
						.file_name()
						.unwrap() // assume the path isn't terminated in ..
						.to_str()
						.map(ToOwned::to_owned)
						.context("File path was not valid unicode")?;

					let cargo_toml = cargo_toml::Manifest::from_slice(
						fs::read_to_string(PathBuf::from_iter([
							dir_entry.path(),
							"Cargo.toml".into(),
						]))
						.context(format!(
							"Error reading Cargo.toml file for pallet {}",
							&pallet_name
						))?
						.as_bytes(),
					)
					.context(format!(
						"Error parsing Cargo.toml file for pallet {}",
						&pallet_name
					))?;

					let lib_rs_path =
						PathBuf::from_iter([dir_entry.path(), "src".into(), "lib.rs".into()])
							.canonicalize()
							.context(format!(
								"Unable to canonicalize path to lib.rs file for pallet {}",
								&pallet_name
							))?;

					lib_rs_path
						.exists()
						.then(|| ())
						.context(format!("Pallet {} does not have a lib.rs file", &pallet_name))?;

					let docs_output_folder =
						PathBuf::from_iter([pallet_docs_output_path, &PathBuf::from(&pallet_name)]);

					if docs_output_folder.exists() {
						Ok(Some(PalletInfo {
							pallet_name_full: cargo_toml
								.package
								.context(format!(
									"Cargo.toml file for pallet {} has no `[package]` section",
									&pallet_name
								))?
								.name,
							docs_output_paths: DocsOutputInfo {
								folder: docs_output_folder.canonicalize().context(format!(
									"Unable to canonicalize path to output folder for pallet {}",
									&pallet_name
								))?,
								extrinsics_docs_file_name: "extrinsics.md".to_string().into(),
							},
							pallet_name,
							lib_rs_path,
						}))
					} else {
						log::warn!(
							"Pallet `{}` does not have a section in the book; skipping",
							&pallet_name
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

#[derive(Debug, Clone)]
pub(crate) struct PalletInfo {
	/// The name of the pallet.
	pub(crate) pallet_name: String,
	/// The full name of the pallet, as defined in the pallet's Cargo.toml `[package.name]` field.
	pub(crate) pallet_name_full: String,
	/// The absolute path to the pallet's `lib.rs` file.
	pub(crate) lib_rs_path: PathBuf,
	/// Where to output the generated documentation to. See [`DocsOutputInfo`] for more
	/// information.
	pub(crate) docs_output_paths: DocsOutputInfo,
}

/// Information regarding where the generated documentation is to be written to.
#[derive(Debug, Clone)]
pub(crate) struct DocsOutputInfo {
	/// The folder to output the generated files to.
	pub(crate) folder: PathBuf,
	/// The name of the extrinsics file.
	pub(crate) extrinsics_docs_file_name: PathBuf,
}

impl DocsOutputInfo {
	/// Returns the absolute path to where the extrinsics documentation is to be written to.
	pub(crate) fn extrinsics_docs_file_path(&self) -> PathBuf {
		PathBuf::from_iter([&self.folder, &self.extrinsics_docs_file_name])
	}
}
