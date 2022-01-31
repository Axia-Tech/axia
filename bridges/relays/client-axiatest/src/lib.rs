// Copyright 2019-2021 AXIA Technologies (UK) Ltd.
// This file is part of AXIA Bridges Common.

// AXIA Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// AXIA Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with AXIA Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Types used to connect to the AXIATest chain.

use relay_substrate_client::{Chain, ChainBase};
use std::time::Duration;

/// AXIATest header id.
pub type HeaderId = relay_utils::HeaderId<bp_axiatest::Hash, bp_axiatest::BlockNumber>;

/// AXIATest chain definition
#[derive(Debug, Clone, Copy)]
pub struct AXIATest;

impl ChainBase for AXIATest {
	type BlockNumber = bp_axiatest::BlockNumber;
	type Hash = bp_axiatest::Hash;
	type Hasher = bp_axiatest::Hasher;
	type Header = bp_axiatest::Header;
}

impl Chain for AXIATest {
	const NAME: &'static str = "AXIATest";
	const AVERAGE_BLOCK_INTERVAL: Duration = Duration::from_secs(6);

	type AccountId = bp_axiatest::AccountId;
	type Index = bp_axiatest::Nonce;
	type SignedBlock = bp_axiatest::SignedBlock;
	type Call = ();
	type Balance = bp_axiatest::Balance;
}

/// AXIATest header type used in headers sync.
pub type SyncHeader = relay_substrate_client::SyncHeader<bp_axiatest::Header>;
