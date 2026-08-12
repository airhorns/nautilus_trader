// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2026 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

//! Polymarket-specific custom data types.
//!
//! These types carry Polymarket RTDS domain data through the Nautilus data engine as
//! [`CustomData`](nautilus_model::data::CustomData).

use nautilus_core::UnixNanos;
use nautilus_model::types::Price;
use nautilus_persistence_macros::custom_data;

/// Canonical Nautilus custom-data name for a main Polymarket market-data transport reconnect.
pub const POLYMARKET_TRANSPORT_RECONNECT_TYPE_NAME: &str = "PolymarketTransportReconnect";

/// Main Polymarket market-data transport reconnect boundary.
///
/// The adapter emits this event before any replacement snapshot or incremental update from the
/// new transport can enter the Nautilus data engine. Consumers can therefore invalidate every
/// application-level book atomically and fail closed until authoritative replacement snapshots
/// arrive. Token-local snapshots caused by fills or tick-size changes do not emit this event.
#[custom_data(pyo3, no_arrow, stub_module = "nautilus_trader.adapters.polymarket")]
pub struct PolymarketTransportReconnect {
    /// UNIX timestamp (nanoseconds) when the reconnect boundary was observed.
    pub ts_event: UnixNanos,
    /// UNIX timestamp (nanoseconds) when the instance was initialized.
    pub ts_init: UnixNanos,
}

/// Polymarket RTDS crypto price sample from the `crypto_prices` topic.
///
/// The adapter normalizes both live `update` frames and `subscribe` backfill
/// snapshots into this per-tick custom data type.
#[custom_data(pyo3, no_arrow, stub_module = "nautilus_trader.adapters.polymarket")]
pub struct PolymarketRtdsCryptoPrice {
    /// Lowercase venue symbol, e.g. `btcusdt`.
    pub symbol: String,
    /// Current spot price.
    #[custom_data_field(serde)]
    pub value: Price,
    /// Price measurement timestamp in Unix milliseconds.
    pub price_timestamp_ms: u64,
    /// RTDS envelope timestamp in Unix milliseconds.
    pub message_timestamp_ms: u64,
    /// UNIX timestamp (nanoseconds) when the price event occurred.
    pub ts_event: UnixNanos,
    /// UNIX timestamp (nanoseconds) when the instance was initialized.
    pub ts_init: UnixNanos,
}

/// Polymarket RTDS equity price sample from the `equity_prices` topic.
///
/// The adapter normalizes both live `update` frames and `subscribe` backfill
/// snapshots into this per-tick custom data type.
#[custom_data(pyo3, no_arrow, stub_module = "nautilus_trader.adapters.polymarket")]
pub struct PolymarketRtdsEquityPrice {
    /// Lowercase venue symbol, e.g. `aapl`, `eurusd`, or `xauusd`.
    pub symbol: String,
    /// Spot price rounded to the venue's float payload precision.
    #[custom_data_field(serde)]
    pub value: Price,
    /// Full-precision spot price when supplied, otherwise the venue's `value`.
    #[custom_data_field(serde)]
    pub full_accuracy_value: Price,
    /// Price measurement timestamp in Unix milliseconds.
    pub price_timestamp_ms: u64,
    /// RTDS envelope timestamp in Unix milliseconds.
    pub message_timestamp_ms: u64,
    /// System receipt timestamp in Unix milliseconds when present.
    #[custom_data_field(serde)]
    pub received_at_ms: Option<u64>,
    /// `true` when the venue is carrying forward the last known value.
    pub is_carried_forward: bool,
    /// UNIX timestamp (nanoseconds) when the price event occurred.
    pub ts_event: UnixNanos,
    /// UNIX timestamp (nanoseconds) when the instance was initialized.
    pub ts_init: UnixNanos,
}

/// Registers Polymarket custom data types.
///
/// Safe to call multiple times (idempotent via internal `Once` guards).
pub fn register_polymarket_custom_data() {
    let _ =
        nautilus_model::data::ensure_custom_data_json_registered::<PolymarketTransportReconnect>();
    let _ = nautilus_model::data::ensure_custom_data_json_registered::<PolymarketRtdsCryptoPrice>();
    let _ = nautilus_model::data::ensure_custom_data_json_registered::<PolymarketRtdsEquityPrice>();
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{POLYMARKET_TRANSPORT_RECONNECT_TYPE_NAME, register_polymarket_custom_data};

    #[rstest]
    fn test_register_polymarket_custom_data_is_idempotent() {
        register_polymarket_custom_data();
        register_polymarket_custom_data();
        assert_eq!(
            POLYMARKET_TRANSPORT_RECONNECT_TYPE_NAME,
            "PolymarketTransportReconnect"
        );
    }
}
