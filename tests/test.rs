use build_time::{build_time_local, build_time_utc};
use chrono::DateTime;
use std::{thread::sleep, time::Duration};

#[test]
fn call_twice() {
    let utc = build_time_utc!();
    let local = build_time_local!();

    sleep(Duration::from_secs(1));

    let utc1 = build_time_utc!();
    let local1 = build_time_local!();

    assert_eq!(utc, utc1);
    assert_eq!(local, local1);
}

#[test]
fn local_utc_match() {
    let utc = DateTime::parse_from_rfc3339(build_time_utc!()).unwrap();
    let local = DateTime::parse_from_rfc3339(build_time_local!()).unwrap();

    assert_eq!(utc, local);
}

#[test]
fn strftime_format() {
    let utc_rfc3339 = build_time_utc!();
    let local_rfc3339 = build_time_local!();

    let utc_formatted = build_time_utc!("%Y-%m-%dT%H:%M:%S%.f%:z");
    let local_formatted = build_time_local!("%Y-%m-%dT%H:%M:%S%.f%:z");

    assert_eq!(utc_rfc3339, utc_formatted);
    assert_eq!(local_rfc3339, local_formatted);
}
