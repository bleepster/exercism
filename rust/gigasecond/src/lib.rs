extern crate chrono;
use chrono::*;

pub fn after(date: DateTime<Utc>) -> DateTime<Utc> {
    let t: i64 = DateTime::<Utc>::timestamp(&date) + 1000000000;
    return DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(t, 0), Utc);
}
