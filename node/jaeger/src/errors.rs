// Copyright 2020 AXIA Technologies (UK) Ltd.
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

//! AXIA Jaeger error definitions.

/// A description of an error during jaeger initialization.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum JaegerError {
	#[error("Already launched the collector thread")]
	AlreadyLaunched,

	#[error("Missing jaeger configuration")]
	MissingConfiguration,
}
