use build_timestamp::{build_timestamp_local, build_timestamp_utc};
use chrono::DateTime;
use std::{thread::sleep, time::Duration};

#[test]
fn call_twice() {
    let utc = build_timestamp_utc!();
    let local = build_timestamp_local!();

    sleep(Duration::from_secs(1));

    let utc1 = build_timestamp_utc!();
    let local1 = build_timestamp_local!();

    assert_eq!(utc, utc1);
    assert_eq!(local, local1);
}

#[test]
fn local_utc_match() {
    let utc = DateTime::parse_from_rfc3339(build_timestamp_utc!()).unwrap();
    let local = DateTime::parse_from_rfc3339(build_timestamp_local!()).unwrap();

    assert_eq!(utc, local);
}
