use color_eyre::eyre;

fn main() -> eyre::Result<()> {
	color_eyre::install()?;
	composable_node::command::run()?;
	Ok(())
}
