// Copyright (C) 2022 ComposableFi.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use once_cell::sync::Lazy;
use std::env;

static RELAY_URL: Lazy<String> = Lazy::new(|| {
	let host = env::var("RELAY_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
	format!("ws://{}:9944", host)
});

static PARA_URL: Lazy<String> = Lazy::new(|| {
	let host = env::var("PARA_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
	format!("ws://{}:9188", host)
});

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
	// TODO(Dzmitry): add conditional env var
	if env::var("SUBXT_ENABLED").is_ok() {
		println!("relay_url: {}, para_url: {}", RELAY_URL.as_str(), &PARA_URL.as_str());
		subxt_codegen::build_script(&PARA_URL, "polkadot").await?;
		subxt_codegen::build_script(&RELAY_URL, "parachain").await?;
	}
	Ok(())
}
