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

use crate::client_state::ClientState;
use alloc::{
	borrow::ToOwned,
	string::{String, ToString},
};
use ibc::{
	core::{ics02_client, ics04_channel, ics24_host::error::ValidationError},
	timestamp::{ParseTimestampError, TimestampOverflowError},
};
use prost::DecodeError;

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(derive_more::From, derive_more::Display)]
pub enum Error {
	Codec(codec::Error),
	TimeStamp(TimestampOverflowError),
	ParseTimeStamp(ParseTimestampError),
	ValidationError(ValidationError),
	Ics02(ics02_client::error::Error),
	Ics04(ics04_channel::error::Error),
	ProtoBuf(DecodeError),
	GrandpaPrimitives(grandpa_client_primitives::error::Error),
	Anyhow(anyhow::Error),
	Custom(String),
}

impl From<Error> for ics02_client::error::Error {
	fn from(e: Error) -> Self {
		ics02_client::error::Error::client_error(
			ClientState::<()>::client_type().to_owned(),
			e.to_string(),
		)
	}
}
