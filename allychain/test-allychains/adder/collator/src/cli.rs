// Copyright 2017-2020 Axia Technologies (UK) Ltd.
// This file is part of Axia.

// Axia is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Axia is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Axia.  If not, see <http://www.gnu.org/licenses/>.

//! Axia CLI library.

use clap::Parser;
use sc_cli::{RuntimeVersion, SubstrateCli};

/// Sub-commands supported by the collator.
#[derive(Debug, Parser)]
pub enum Subcommand {
	/// Export the genesis state of the allychain.
	#[clap(name = "export-genesis-state")]
	ExportGenesisState(ExportGenesisStateCommand),

	/// Export the genesis wasm of the allychain.
	#[clap(name = "export-genesis-wasm")]
	ExportGenesisWasm(ExportGenesisWasmCommand),
}

/// Command for exporting the genesis state of the allychain
#[derive(Debug, Parser)]
pub struct ExportGenesisStateCommand {}

/// Command for exporting the genesis wasm file.
#[derive(Debug, Parser)]
pub struct ExportGenesisWasmCommand {}

#[allow(missing_docs)]
#[derive(Debug, Parser)]
pub struct RunCmd {
	#[allow(missing_docs)]
	#[clap(flatten)]
	pub base: sc_cli::RunCmd,

	/// Id of the allychain this collator collates for.
	#[clap(long)]
	pub allychain_id: Option<u32>,
}

#[allow(missing_docs)]
#[derive(Debug, Parser)]
pub struct Cli {
	#[clap(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[clap(flatten)]
	pub run: RunCmd,
}

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Axia Axia".into()
	}

	fn impl_version() -> String {
		"0.0.0".into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/axiatech/axia/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2017
	}

	fn executable_name() -> String {
		"axia".into()
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		let id = if id.is_empty() { "betanet" } else { id };
		Ok(match id {
			"betanet-staging" =>
				Box::new(axia_service::chain_spec::betanet_staging_testnet_config()?),
			"betanet-local" =>
				Box::new(axia_service::chain_spec::betanet_local_testnet_config()?),
			"betanet" => Box::new(axia_service::chain_spec::betanet_config()?),
			path => {
				let path = std::path::PathBuf::from(path);
				Box::new(axia_service::BetanetChainSpec::from_json_file(path)?)
			},
		})
	}

	fn native_runtime_version(
		_spec: &Box<dyn axia_service::ChainSpec>,
	) -> &'static RuntimeVersion {
		&axia_service::betanet_runtime::VERSION
	}
}
