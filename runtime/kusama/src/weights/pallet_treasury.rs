// Copyright 2017-2021 Parity Technologies (UK) Ltd.
// This file is part of AXIA.

// AXIA is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// AXIA is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with AXIA.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_treasury`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-10-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 128

// Executed Command:
// target/release/axia
// benchmark
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=pallet_treasury
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/pallet_treasury.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_treasury`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_treasury::WeightInfo for WeightInfo<T> {
	// Storage: Treasury ProposalCount (r:1 w:1)
	// Storage: Treasury Proposals (r:0 w:1)
	fn propose_spend() -> Weight {
		(36_366_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Treasury Proposals (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn reject_proposal() -> Weight {
		(56_424_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Treasury Proposals (r:1 w:0)
	// Storage: Treasury Approvals (r:1 w:1)
	fn approve_proposal(p: u32, ) -> Weight {
		(11_436_000 as Weight)
			// Standard Error: 0
			.saturating_add((32_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Treasury Approvals (r:1 w:1)
	// Storage: Bounties BountyApprovals (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Treasury Proposals (r:2 w:2)
	fn on_initialize_proposals(p: u32, ) -> Weight {
		(76_150_000 as Weight)
			// Standard Error: 37_000
			.saturating_add((57_755_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(p as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(p as Weight)))
	}
}
