use std::time::{SystemTime, UNIX_EPOCH};
use common::TimestampType;

pub fn get_current_timestamp() -> TimestampType {
    let now = SystemTime::now();

    let since_the_epoch = now.duration_since(UNIX_EPOCH).unwrap();

    since_the_epoch.as_millis()
}
