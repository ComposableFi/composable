#![allow(clippy::disallowed_methods)] // temporarilly

use std::{env, fmt::Write, fs, iter, path::PathBuf};

use syn::{Generics, Item, ItemFn, ItemMod, ReturnType, Signature, Visibility};

fn main() {
	aggregate_tests("testing");
}

pub fn aggregate_tests(tests_module_path: &str) {
	if env::var_os("SKIP_TESTS_BUILD_SCRIPT").is_some() {
		println!("cargo:rerun-if-env-changed=SKIP_TESTS_BUILD_SCRIPT");
		return
	}

	let dest_path = PathBuf::from(
		env::var_os("OUT_DIR").expect("OUT_DIR environment variable should be present"),
	)
	.join("testing.rs");

	// "testing"
	let ast = match read_module_file(vec![tests_module_path.to_string()]) {
		Ok(ast) => ast,
		Err(path) => {
			rerun_if_changed(&path, true);
			return
		},
	};

	let Ok(test_fns) = ast
			.items
			.into_iter()
			.filter_map(|item| to_output(item, "testing".to_string()))
			.collect::<Result<Vec<_>, _>>()
		else {
			rerun_if_changed(tests_module_path, true);
			return;
		};

	fs::write(
		dest_path,
		format!(
			"
				#[macro_export]
				macro_rules! tests {{
					(mod $crate_name:ident<$Runtime:ty>) => {{
						mod $crate_name {{
							{}
						}}
					}}
				}}
			",
			// pub use tests;
			test_fns
				.into_iter()
				.fold(String::new(), |acc, curr| acc + &curr.print(tests_module_path.to_string())),
		),
	)
	.unwrap();

	rerun_if_changed(tests_module_path, true);
}

fn rerun_if_changed(path: &str, glob: bool) {
	println!("cargo:rerun-if-changed=src/{}{}", path, if glob { "*" } else { "" });
}

fn read_module_file(paths: Vec<String>) -> Result<syn::File, String> {
	let path = format!("src/{}.rs", paths.join("/"));

	let Ok(content) = fs::read_to_string(&path) else {
	    return Err(path);
    };

	let Ok(ast) = syn::parse_file(&content) else {
	    return Err(path);
    };

	Ok(ast)
}

// TODO(benluelo): Rename this?
enum AccumulatedTests {
	Mod(String, Vec<AccumulatedTests>),
	Fn(String),
}

impl AccumulatedTests {
	fn print(self, crate_name: String) -> String {
		fn print_inner(output: AccumulatedTests, parent_mods: &mut Vec<String>) -> String {
			match output {
				AccumulatedTests::Mod(ident, content) => {
					let mut output = String::new();

					output.write_fmt(format_args!("mod {ident} {{")).unwrap();

					parent_mods.push(ident.clone());

					output = content
						.into_iter()
						.fold(output, |acc, output| acc + &print_inner(output, parent_mods));

					// temporary check
					assert_eq!(parent_mods.pop().unwrap(), ident);

					output.write_char('}').unwrap();

					output
				},
				AccumulatedTests::Fn(ident) => format!(
					"
					#[test] fn {ident}() {{
						::sp_io::TestExternalities::default().execute_with(|| {{
							$crate{}::{ident}::<$Runtime>();
						}})
					}}",
					parent_mods
						.iter()
						.map(|path_segment| format!("::{path_segment}"))
						.collect::<String>()
				),
			}
		}

		print_inner(self, &mut vec![crate_name])
	}
}

fn to_output(item: Item, root_dir: String) -> Option<Result<AccumulatedTests, String>> {
	to_output_inner(item, vec![root_dir])
}

// TODO(benluelo): Pass attrs through, maybe check for non-test/cfg(test) fns/mods?
fn to_output_inner(
	item: Item,
	current_paths: Vec<String>,
) -> Option<Result<AccumulatedTests, String>> {
	match item {
		Item::Fn(item_fn) => match item_fn {
			ItemFn {
				attrs,
				vis: Visibility::Public(_),
				sig:
					Signature {
						constness: None,
						asyncness: None,
						unsafety: None,
						abi: None,
						ident,
						fn_token: _,
						generics: Generics { lt_token: _, params, gt_token: _, where_clause: _ },
						paren_token: _,
						inputs,
						variadic: None,
						output: ReturnType::Default,
						..
					},
				block: _,
			} if attrs.is_empty() && inputs.is_empty() && params.len() == 1 =>
				Some(Ok(AccumulatedTests::Fn(ident.to_string()))),
			_ => None,
		},
		Item::Mod(ItemMod {
			attrs,
			vis: Visibility::Public(_),
			mod_token: _,
			ident,
			content,
			semi: _,
		}) if attrs.is_empty() => {
			let current_paths_with_self = current_paths
				.into_iter()
				.chain(iter::once(ident.to_string()))
				.collect::<Vec<_>>();

			let content = match content {
				// empty declaration, content is in another file
				// TODO(benluelo): Support renamed module paths (path = "file.rs")
				None => match read_module_file(current_paths_with_self.clone()) {
					Ok(ok) => ok.items,
					Err(path) => return Some(Err(path)),
				},
				Some((_, content)) => content,
			};

			Some(Ok(AccumulatedTests::Mod(
				ident.to_string(),
				match content
					.into_iter()
					.flat_map(|item| to_output_inner(item, current_paths_with_self.clone()))
					.collect::<Result<Vec<_>, _>>()
				{
					Ok(ok) => ok,
					Err(err) => return Some(Err(err)),
				},
			)))
		},
		_ => None,
	}
}
