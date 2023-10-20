use chrono::{DateTime, Utc, Duration};

const GIGASECOND: i64 = 1000000000;
// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {

    let duration = Duration::seconds(GIGASECOND);

    let after = match start.checked_add_signed(duration) {
        Some(a) => a,
        None => start
    };

    after
}
