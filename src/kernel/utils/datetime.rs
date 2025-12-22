use chrono::{Local, NaiveDateTime};

pub fn start_of_today() -> NaiveDateTime {
    Local::now().date_naive().and_hms_opt(0, 0, 0).unwrap()
}
