use chrono::{DateTime, Utc};
use imu_fusion::{Fusion, FusionAhrsSettings, FusionEuler, FusionVector};

use crate::Measurement;

const SAMPLE_RATE_HZ: u32 = 125;

pub struct Model {
    fusion: Fusion,
    start_timestamp: Option<DateTime<Utc>>,
}

impl Model {
    pub fn update(&mut self, measurement: &Measurement) {
        if self.start_timestamp.is_none() {
            self.start_timestamp = Some(measurement.timestamp);
            return;
        }

        let date_time: f32 = (self.start_timestamp.unwrap().timestamp_micros()
            - measurement.timestamp.timestamp_micros()) as f32
            / 1e6;
        let gyro_vector = FusionVector::new(measurement.gy_x, measurement.gy_y, measurement.gy_z);
        let acceleration_vector =
            FusionVector::new(measurement.acc_x, measurement.acc_y, measurement.acc_z);
        self.fusion
            .update_no_mag(gyro_vector, acceleration_vector, date_time);
    }

    pub fn euler(&self) -> FusionEuler {
        self.fusion.euler()
    }

    pub fn new() -> Model {
        let ahrs_settings = FusionAhrsSettings::new();
        let fusion = imu_fusion::Fusion::new(SAMPLE_RATE_HZ, ahrs_settings);
        Model {
            fusion,
            start_timestamp: None,
        }
    }
}
