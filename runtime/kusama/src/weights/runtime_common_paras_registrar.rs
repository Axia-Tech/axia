// Copyright 2017-2021 AXIA Technologies (UK) Ltd.
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
//! Autogenerated weights for `runtime_common::paras_registrar`
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
// --pallet=runtime_common::paras_registrar
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/runtime_common_paras_registrar.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_common::paras_registrar`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_common::paras_registrar::WeightInfo for WeightInfo<T> {
	// Storage: Registrar NextFreeParaId (r:1 w:1)
	// Storage: Registrar Paras (r:1 w:1)
	// Storage: Paras ParaLifecycles (r:1 w:0)
	fn reserve() -> Weight {
		(42_316_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Registrar Paras (r:1 w:1)
	// Storage: Paras ParaLifecycles (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Paras ActionsQueue (r:1 w:1)
	// Storage: Paras UpcomingParasGenesis (r:0 w:1)
	fn register() -> Weight {
		(7_215_643_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Registrar Paras (r:1 w:1)
	// Storage: Paras ParaLifecycles (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Paras ActionsQueue (r:1 w:1)
	// Storage: Paras UpcomingParasGenesis (r:0 w:1)
	fn force_register() -> Weight {
		(7_199_236_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Registrar Paras (r:1 w:1)
	// Storage: Paras ParaLifecycles (r:1 w:1)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Paras ActionsQueue (r:1 w:1)
	// Storage: Registrar PendingSwap (r:0 w:1)
	fn deregister() -> Weight {
		(73_245_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Registrar Paras (r:1 w:0)
	// Storage: Registrar PendingSwap (r:1 w:1)
	// Storage: Paras ParaLifecycles (r:2 w:2)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Paras ActionsQueue (r:1 w:1)
	// Storage: Crowdloan Funds (r:2 w:2)
	// Storage: Slots Leases (r:2 w:2)
	fn swap() -> Weight {
		(62_394_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
}
