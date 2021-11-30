// Copyright 2021 AXIA Technologies (UK) Ltd.
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
//

//! Error handling related code and Error/Result definitions.

use thiserror::Error;

use axia_node_subsystem_util::runtime;
use axia_subsystem::SubsystemError;

use crate::{sender, LOG_TARGET};

#[derive(Debug, Error, derive_more::From)]
#[error(transparent)]
pub enum Error {
	/// Fatal errors of dispute distribution.
	Fatal(Fatal),
	/// Non-fatal errors of dispute distribution.
	NonFatal(NonFatal),
}

impl From<sender::Error> for Error {
	fn from(o: sender::Error) -> Self {
		match o {
			sender::Error::Fatal(f) => Self::Fatal(Fatal::Sender(f)),
			sender::Error::NonFatal(f) => Self::NonFatal(NonFatal::Sender(f)),
		}
	}
}

/// Fatal errors of this subsystem.
#[derive(Debug, Error)]
pub enum Fatal {
	/// Receiving subsystem message from overseer failed.
	#[error("Receiving message from overseer failed")]
	SubsystemReceive(#[source] SubsystemError),

	/// Spawning a running task failed.
	#[error("Spawning subsystem task failed")]
	SpawnTask(#[source] SubsystemError),

	/// `DisputeSender` mpsc receiver exhausted.
	#[error("Erasure chunk requester stream exhausted")]
	SenderExhausted,

	/// Errors coming from `runtime::Runtime`.
	#[error("Error while accessing runtime information")]
	Runtime(#[from] runtime::Fatal),

	/// Errors coming from `DisputeSender`
	#[error("Error while accessing runtime information")]
	Sender(#[from] sender::Fatal),
}

/// Non-fatal errors of this subsystem.
#[derive(Debug, Error)]
pub enum NonFatal {
	/// Errors coming from `DisputeSender`
	#[error("Error while accessing runtime information")]
	Sender(#[from] sender::NonFatal),
}

pub type Result<T> = std::result::Result<T, Error>;

pub type FatalResult<T> = std::result::Result<T, Fatal>;

/// Utility for eating top level errors and log them.
///
/// We basically always want to try and continue on error. This utility function is meant to
/// consume top-level errors by simply logging them
pub fn log_error(result: Result<()>, ctx: &'static str) -> std::result::Result<(), Fatal> {
	match result {
		Err(Error::Fatal(f)) => Err(f),
		Err(Error::NonFatal(error)) => {
			tracing::warn!(target: LOG_TARGET, error = ?error, ctx);
			Ok(())
		},
		Ok(()) => Ok(()),
	}
}
