// Copyright (C) 2022 Quickwit, Inc.
//
// Quickwit is offered under the AGPL v3.0 and as commercial software.
// For commercial licensing, contact us at hello@quickwit.io.
//
// AGPL:
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

mod checklist;
mod coolid;

pub mod fs;
pub mod metrics;
pub mod net;
pub mod rand;
pub mod runtimes;
pub mod uri;

use std::fmt::Debug;
use std::ops::Range;
use std::str::FromStr;

pub use checklist::{print_checklist, run_checklist, BLUE_COLOR, GREEN_COLOR, RED_COLOR};
pub use coolid::new_coolid;
use tracing::{error, info};

pub fn chunk_range(range: Range<usize>, chunk_size: usize) -> impl Iterator<Item = Range<usize>> {
    range.clone().step_by(chunk_size).map(move |block_start| {
        let block_end = (block_start + chunk_size).min(range.end);
        block_start..block_end
    })
}

pub fn into_u64_range(range: Range<usize>) -> Range<u64> {
    range.start as u64..range.end as u64
}

pub fn setup_logging_for_tests() {
    let _ = env_logger::builder().format_timestamp(None).try_init();
}

pub fn split_file(split_id: &str) -> String {
    format!("{}.split", split_id)
}

pub fn get_from_env<T: FromStr + Debug>(key: &str, default_value: T) -> T {
    if let Ok(value_str) = std::env::var(key) {
        if let Ok(value) = T::from_str(&value_str) {
            info!(value=?value, "Setting `{}` from environment", key);
            return value;
        } else {
            error!(value_str=%value_str, "Failed to parse `{}` from environment", key);
        }
    }
    info!(value=?default_value, "Setting `{}` from default", key);
    default_value
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_from_env() {
        const TEST_KEY: &str = "TEST_KEY";
        assert_eq!(super::get_from_env(TEST_KEY, 10), 10);
        std::env::set_var(TEST_KEY, "15");
        assert_eq!(super::get_from_env(TEST_KEY, 10), 15);
        std::env::set_var(TEST_KEY, "1invalidnumber");
        assert_eq!(super::get_from_env(TEST_KEY, 10), 10);
    }
}
