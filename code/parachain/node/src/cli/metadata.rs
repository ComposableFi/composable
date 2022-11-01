use clap::Parser;
use sc_cli::{CliConfiguration, Error, ImportParams, SharedParams};
use sp_api::{Metadata, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_runtime::generic::BlockId;
use std::{fmt::Debug, io, io::Write, sync::Arc};

/// Command for exporting the metadata for a runtime.
#[derive(Debug, Parser)]
pub struct MetadataCmd {
	#[allow(missing_docs)]
	#[clap(flatten)]
	pub shared_params: SharedParams,

	#[allow(missing_docs)]
	#[clap(flatten)]
	pub import_params: ImportParams,
}

impl MetadataCmd {
	pub fn run<B, C>(&self, client: Arc<C>) -> Result<(), Error>
	where
		B: sp_runtime::traits::Block,
		C: ProvideRuntimeApi<B> + HeaderBackend<B> + Send + Sync + 'static,
		C::Api: Metadata<B>,
	{
		let info = client.info();
		let metadata = client
			.runtime_api()
			.metadata(&BlockId::Hash(info.best_hash))
			.map_err(|err| Error::Application(Box::new(err)))?;

		io::stdout().write_all(&metadata)?;

		Ok(())
	}
}

impl CliConfiguration for MetadataCmd {
	fn shared_params(&self) -> &SharedParams {
		&self.shared_params
	}

	fn import_params(&self) -> Option<&ImportParams> {
		Some(&self.import_params)
	}
}
