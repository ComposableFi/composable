use crate::error::Error;
use clap::Args;
use std::process::Command;

const CARGO_GENERATE_COMMAND_NAME: &str = "generate";
const CARGO_GENERATE_PROJECT_NAME: &str = "cargo-generate";
const CARGO_GENERATE_VERSION: &str = "0.17.0";

const CARGO_TEMPLATE_KEY_PROJECT_DESCRIPTION: &str = "project_description";
const CARGO_TEMPLATE_KEY_PROJECT_AUTHORS: &str = "project_authors";
const CARGO_TEMPLATE_GIT: &str = "https://github.com/ComposableFi/cw-template-project";

#[derive(Args, Debug)]
/// Interact with a substrate-based chain.
pub struct NewCommand {
	/// Project name
	#[arg(short, long)]
	name: Option<String>,

	/// Project description
	#[arg(short, long)]
	description: Option<String>,

	/// Project authors
	#[arg(short, long)]
	author: Option<String>,
}

impl NewCommand {
	pub fn run(self) -> Result<(), Error> {
		self.install_cargo_generate()?;

		let mut command = Command::new("cargo");

		let _ = command.args(&[CARGO_GENERATE_COMMAND_NAME, "--git", CARGO_TEMPLATE_GIT]);

		if let Some(name) = self.name {
			let _ = command.arg("-n").arg(&name);
		}

		if let Some(description) = self.description {
			let _ = command
				.arg("-d")
				.arg(&format!("{}={}", CARGO_TEMPLATE_KEY_PROJECT_DESCRIPTION, description));
		}

		if let Some(author) = self.author {
			let _ = command
				.arg("-d")
				.arg(&format!("{}={}", CARGO_TEMPLATE_KEY_PROJECT_AUTHORS, author));
		}

		let status = command.status().map_err(|e| Error::Internal(Box::new(e)))?;

		if status.success() {
			Ok(())
		} else {
			Err(Error::ShellCommandFailure)
		}
	}

	fn check_if_generate_installed(&self) -> Result<bool, Error> {
		Ok(Command::new("cargo")
			.arg(CARGO_GENERATE_COMMAND_NAME)
			.arg("-V")
			.status()
			.map_err(|e| Error::Internal(Box::new(e)))?
			.success())
	}

	fn install_cargo_generate(&self) -> Result<(), Error> {
		if self.check_if_generate_installed()? {
			return Ok(())
		}

		let status = Command::new("cargo")
			.arg("install")
			.arg(CARGO_GENERATE_PROJECT_NAME)
			.arg("--version")
			.arg(CARGO_GENERATE_VERSION)
			.status()
			.map_err(|e| Error::Internal(Box::new(e)))?;

		if status.success() {
			Ok(())
		} else {
			Err(Error::ShellCommandFailure)
		}
	}
}
