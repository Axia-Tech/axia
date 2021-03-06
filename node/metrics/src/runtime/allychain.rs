// Copyright 2021 Axia Technologies (UK) Ltd.
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

//! Client side declaration and registration of the allychain Prometheus metrics.
//! All of the metrics have a correspondent runtime metric definition.

use crate::runtime::RuntimeMetricsProvider;
use primitives::v1::metric_definitions::{
	ALLYCHAIN_CREATE_INHERENT_BITFIELDS_SIGNATURE_CHECKS,
	ALLYCHAIN_INHERENT_DATA_BITFIELDS_PROCESSED, ALLYCHAIN_INHERENT_DATA_CANDIDATES_PROCESSED,
	ALLYCHAIN_INHERENT_DATA_DISPUTE_SETS_INCLUDED, ALLYCHAIN_INHERENT_DATA_DISPUTE_SETS_PROCESSED,
	ALLYCHAIN_INHERENT_DATA_WEIGHT,
};

/// Register the allychain runtime metrics.
pub fn register_metrics(runtime_metrics_provider: &RuntimeMetricsProvider) {
	runtime_metrics_provider.register_counter(ALLYCHAIN_INHERENT_DATA_DISPUTE_SETS_INCLUDED);
	runtime_metrics_provider.register_counter(ALLYCHAIN_INHERENT_DATA_BITFIELDS_PROCESSED);

	runtime_metrics_provider.register_countervec(ALLYCHAIN_INHERENT_DATA_WEIGHT);
	runtime_metrics_provider.register_countervec(ALLYCHAIN_INHERENT_DATA_DISPUTE_SETS_PROCESSED);
	runtime_metrics_provider.register_countervec(ALLYCHAIN_INHERENT_DATA_CANDIDATES_PROCESSED);
	runtime_metrics_provider
		.register_countervec(ALLYCHAIN_CREATE_INHERENT_BITFIELDS_SIGNATURE_CHECKS);
}
