use crate::error::Error;
use anyhow::anyhow;
use clap::Args;
use std::process::Command;

const CARGO_GENERATE_COMMAND_NAME: &str = "generate";
const CARGO_TEMPLATE_KEY_PROJECT_DESCRIPTION: &str = "project_description";
const CARGO_TEMPLATE_GIT: &str = "https://github.com/ComposableFi/cw-template-project";
const CARGO_GENERATE_HELP_MESSAGE: &str = r#"
cargo-generate is not installed. Please install it by running `cargo install cargo-generate`.
See more at: https://github.com/cargo-generate/cargo-generate
"#;

#[derive(Args, Debug)]
/// Create a base CosmWasm project.
pub struct NewCommand {
	/// Project name
	#[arg(short, long)]
	name: Option<String>,

	/// Project description
	#[arg(short, long)]
	description: Option<String>,
}

impl NewCommand {
	pub fn run(self) -> anyhow::Result<()> {
		self.check_if_generate_installed()?;

		let mut command = Command::new("cargo");

		let _ = command.args([CARGO_GENERATE_COMMAND_NAME, "--git", CARGO_TEMPLATE_GIT]);

		if let Some(name) = self.name {
			let _ = command.arg("-n").arg(&name);
		}

		if let Some(description) = self.description {
			let _ = command
				.arg("-d")
				.arg(&format!("{}={}", CARGO_TEMPLATE_KEY_PROJECT_DESCRIPTION, description));
		}

		let status = command.status()?;

		if status.success() {
			Ok(())
		} else {
			Err(anyhow!("{}", Error::ShellCommandFailure))
		}
	}

	fn check_if_generate_installed(&self) -> anyhow::Result<()> {
		if Command::new("cargo")
			.arg(CARGO_GENERATE_COMMAND_NAME)
			.arg("-V")
			.status()?
			.success()
		{
			Ok(())
		} else {
			Err(anyhow!("{}", Error::ToolNotInstalled(CARGO_GENERATE_HELP_MESSAGE.into())))
		}
	}
}
