use chrono::prelude::*;
use chrono::{Duration, Utc};
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let start_time = Utc.ymd(1970, 1, 1).and_hms(0, 0, 0);

        let mut final_time = start_time + Duration::hours(hours as i64);
        final_time = final_time + Duration::minutes(minutes as i64);

        Clock {
            hours: final_time.hour() as i32,
            minutes: final_time.minute() as i32,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let start_time = Utc
            .ymd(1970, 1, 1)
            .and_hms(self.hours as u32, self.minutes as u32, 0);
        let added_minutes = start_time + Duration::minutes(minutes as i64);

        Clock {
            hours: added_minutes.hour() as i32,
            minutes: added_minutes.minute() as i32,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = 2;
        write!(
            f,
            "{}:{}",
            format!("{:0width$}", self.hours, width = width),
            format!("{:0width$}", self.minutes, width = width)
        )
    }
}
