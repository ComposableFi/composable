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

use std::env;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
	// TODO(Dzmitry): add conditional env var
	if env::var("SUBXT_ENABLED").is_ok() {
		subxt_codegen::build_script("ws://127.0.0.1:9944", "polkadot").await?;
		subxt_codegen::build_script("ws://127.0.0.1:9188", "parachain").await?;
	}
	Ok(())
}
