// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
#![deny(unused_extern_crates, missing_docs)]

//! Basic example of end to end runtime tests.
mod chain_info;
mod node;
mod sproof;

pub use chain_info::*;
use picasso_runtime::Event;
use sc_client_api::{call_executor::ExecutorProvider, CallExecutor};
use sp_blockchain::HeaderBackend;
use sp_runtime::generic::BlockId;
use sp_runtime::AccountId32;
use std::error::Error;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
	node::run(|node| async move {
		let from = AccountId32::from_str("5uAfQTqudXnnSgSMPVowwRjgNFxBDW2d5AQXP2vHDHy2yJ4w")?;

		node.submit_extrinsic(
			frame_system::Call::remark { remark: b"Hello World".to_vec() },
			Some(from),
		)
		.await?;
		node.seal_blocks(1).await;

		let old_runtime_version = node
			.client()
			.executor()
			.runtime_version(&BlockId::Hash(node.client().info().best_hash))?
			.spec_version;
		println!("\n\nold_runtime_version: {}\n\n", old_runtime_version);

		let code = picasso_runtime::WASM_BINARY
			.ok_or("Polkadot development wasm not available")?
			.to_vec();

		let call = sudo::Call::sudo_unchecked_weight {
			call: Box::new(frame_system::Call::set_code { code }.into()),
			weight: 0,
		};
		// let su = AccountId32::from_str("5z93WG1Lz47b8AjtVbMaLC4M8rohecXPMSjRDBaMUAbmeCi7")?;
		let su = AccountId32::from_str("5uAfQTqudXnnSgSMPVowwRjgNFxBDW2d5AQXP2vHDHy2yJ4w")?;
		node.submit_extrinsic(call, Some(su)).await?;
		node.seal_blocks(2).await;
		// assert that the runtime has been updated by looking at events
		let events = node
			.events()
			.into_iter()
			.filter(|event| match event.event {
				Event::ParachainSystem(parachain_system::Event::ValidationFunctionApplied(_)) => {
					true
				}
				_ => false,
			})
			.collect::<Vec<_>>();
		// make sure event was emitted
		assert_eq!(
			events.len(),
			1,
			"system::Event::CodeUpdate not found in events: {:#?}",
			node.events()
		);
		let new_runtime_version = node
			.client()
			.executor()
			.runtime_version(&BlockId::Hash(node.client().info().best_hash))?
			.spec_version;
		println!("\n\nnew_runtime_version: {}\n\n", new_runtime_version);

		// just confirming
		assert!(
			new_runtime_version > old_runtime_version,
			"Invariant, spec_version of new runtime: {} not greater than spec_version of old runtime: {}",
			new_runtime_version,
			old_runtime_version,
		);
		node.until_shutdown().await;
		Ok(())
	})
}
