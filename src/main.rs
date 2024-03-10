use std::fs::File;

use chrono::{serde::ts_microseconds, Utc};

mod model;

#[derive(Debug, serde::Deserialize)]
pub struct Measurement {
    #[serde(with = "ts_microseconds")]
    timestamp: chrono::DateTime<Utc>,
    acc_x: f32,
    acc_y: f32,
    acc_z: f32,
    gy_x: f32,
    gy_y: f32,
    gy_z: f32,
}

// The `main` function is where your program starts executing.
fn main() {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(File::open("./measurements.csv").unwrap());

    let mut model = model::Model::new();
    let mut count = 0;
    // Loop over each record.
    for result in rdr.deserialize() {
        count += 1;
        // An error may occur, so abort the program in an unfriendly way.
        // We will make this more friendly later!
        let record: Measurement = result.expect("a CSV record");
        model.update(&record);
        let euler = model.euler();
        if count % 12 == 0 {
            // Print a debug version of the record.
            println!(
                "ts: {:?}, angle: ({}, {} , {})  ",
                record.timestamp.timestamp_micros(), euler.angle.pitch, euler.angle.roll, euler.angle.yaw
            );
        }
    }
}
