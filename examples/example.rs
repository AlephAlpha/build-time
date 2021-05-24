use build_timestamp::{build_timestamp_local, build_timestamp_utc};
use std::{thread::sleep, time::Duration};

fn main() {
    println!("Build time UTC: {:?}", build_timestamp_utc!());
    println!("Build time Local: {:?}", build_timestamp_local!());

    sleep(Duration::from_secs(1));

    println!("Build time UTC: {:?}", build_timestamp_utc!());
    println!("Build time Local: {:?}", build_timestamp_local!());
}
