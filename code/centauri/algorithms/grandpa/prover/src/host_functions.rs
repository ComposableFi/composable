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

use crate::H256;
use ics10_grandpa::client_message::RelayChainHeader;
use primitives::HostFunctions;
use sp_core::ed25519::{Public, Signature};
use sp_runtime::{
	app_crypto::RuntimePublic,
	traits::{BlakeTwo256, Header},
};
use std::fmt::Debug;

/// Only holds implementations for the relevant Host Functions for the verifier
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct HostFunctionsProvider;

impl light_client_common::HostFunctions for HostFunctionsProvider {
	type BlakeTwo256 = BlakeTwo256;
}

impl HostFunctions for HostFunctionsProvider {
	type Header = RelayChainHeader;

	fn ed25519_verify(sig: &Signature, msg: &[u8], pubkey: &Public) -> bool {
		pubkey.verify(&msg, sig)
	}

	fn add_relaychain_header_hashes(headers: &[<Self::Header as Header>::Hash]) {
		todo!()
	}

	fn exists_relaychain_header_hash(hash: <Self::Header as Header>::Hash) -> bool {
		todo!()
	}
}
