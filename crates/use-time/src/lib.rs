#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::time::Duration;

pub mod prelude;

pub const MILLISECONDS_PER_SECOND: u64 = 1_000;
pub const SECONDS_PER_MINUTE: u64 = 60;
pub const MINUTES_PER_HOUR: u64 = 60;
pub const SECONDS_PER_HOUR: u64 = SECONDS_PER_MINUTE * MINUTES_PER_HOUR;

#[must_use]
pub fn milliseconds_to_duration(milliseconds: u64) -> Duration {
    Duration::from_millis(milliseconds)
}

#[must_use]
pub fn seconds_to_duration(seconds: u64) -> Duration {
    Duration::from_secs(seconds)
}

#[must_use]
pub fn minutes_to_seconds(minutes: u64) -> Option<u64> {
    minutes.checked_mul(SECONDS_PER_MINUTE)
}

#[must_use]
pub fn hours_to_seconds(hours: u64) -> Option<u64> {
    hours.checked_mul(SECONDS_PER_HOUR)
}

#[must_use]
pub fn duration_to_milliseconds(duration: Duration) -> Option<u64> {
    u64::try_from(duration.as_millis()).ok()
}

#[must_use]
pub fn duration_to_seconds_f64(duration: Duration) -> f64 {
    duration.as_secs_f64()
}

#[must_use]
pub fn split_seconds(total_seconds: u64) -> (u64, u8, u8) {
    let hours = total_seconds / SECONDS_PER_HOUR;
    let remaining_after_hours = total_seconds % SECONDS_PER_HOUR;
    let minutes = remaining_after_hours / SECONDS_PER_MINUTE;
    let seconds = remaining_after_hours % SECONDS_PER_MINUTE;

    (hours, minutes as u8, seconds as u8)
}

#[cfg(test)]
mod tests {
    use super::{
        duration_to_milliseconds, duration_to_seconds_f64, hours_to_seconds,
        milliseconds_to_duration, minutes_to_seconds, seconds_to_duration, split_seconds,
    };

    #[test]
    fn converts_minutes_and_hours_to_seconds() {
        assert_eq!(minutes_to_seconds(5), Some(300));
        assert_eq!(hours_to_seconds(2), Some(7_200));
    }

    #[test]
    fn builds_and_reads_duration_values() {
        let duration = milliseconds_to_duration(1_250);

        assert_eq!(duration.as_secs(), 1);
        assert_eq!(duration.subsec_millis(), 250);
        assert_eq!(duration_to_milliseconds(duration), Some(1_250));

        let seconds = seconds_to_duration(3);
        assert_eq!(duration_to_seconds_f64(seconds), 3.0);
    }

    #[test]
    fn splits_seconds_into_clock_like_components() {
        assert_eq!(split_seconds(59), (0, 0, 59));
        assert_eq!(split_seconds(3_661), (1, 1, 1));
    }
}
