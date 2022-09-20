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

#[derive(sp_std::fmt::Debug, derive_more::From)]
pub enum Error {
	/// subxt error
	Subxt(subxt::BasicError),
	/// subxt rpc error
	SubxtRRpc(subxt::rpc::RpcError),
	/// Trie error
	TrieProof(Box<sp_trie::TrieError<sp_trie::LayoutV0<sp_runtime::traits::BlakeTwo256>>>),
	/// Custom
	Custom(String),
	/// Codec error
	Codec(codec::Error),
}
