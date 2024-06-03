use chrono::{DateTime, Local, NaiveTime, Utc};
use std::{f32::consts::TAU, time::SystemTime};

use crate::error::Error;

#[derive(Debug)]
pub struct Clock {
    pub tz: f32,
    pub rotation: f32,
}

impl Clock {
    pub fn get_time(&self) -> eyre::Result<ClockValues> {
        let midnight =
            NaiveTime::from_hms_opt(0, 0, 0).ok_or(Error("could not get midnight".to_owned()))?;
        let time = Utc::now().time() - midnight;
        let time = (time.num_milliseconds() as f32) / 1_000.0;
        let sec = time % 60.0;
        let min = (time / 60.0) % 60.0;
        let hour = (time / 3600.0) % 12.0;
        Ok(ClockValues { hour, min, sec })
    }

    pub fn get_angles(&self) -> eyre::Result<ClockValues> {
        let (hour, min, sec) = self.get_time()?.into();
        let sec = Self::get_angle(sec, 60.0);
        let min = Self::get_angle(min, 60.0);
        let hour = Self::get_angle(hour, 12.0);
        Ok(ClockValues { hour, min, sec })
    }

    #[inline]
    fn get_angle(value: f32, max_value: f32) -> f32 {
        let offset = -TAU / 4.0;
        value * TAU / max_value + offset
    }
}

impl Default for Clock {
    fn default() -> Self {
        let tz: DateTime<Local> = SystemTime::now().into();
        let tz = tz.offset().local_minus_utc() as f32 / 3600.0;
        let rotation = -360.0 * tz / 12.0; // Raylib DrawTexturePro uses degrees instead of radians
        Self { tz, rotation }
    }
}

pub struct ClockValues {
    pub hour: f32,
    pub min: f32,
    pub sec: f32,
}

impl From<ClockValues> for (f32, f32, f32) {
    #[inline]
    fn from(val: ClockValues) -> Self {
        (val.hour, val.min, val.sec)
    }
}
