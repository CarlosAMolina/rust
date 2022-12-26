#![allow(unused)]
fn main() {
use time::{Date, PrimitiveDateTime, OffsetDateTime, UtcOffset, macros::{date, datetime, time}};
use time::Weekday::Wednesday;

let date = Date::from_iso_week_date(2022, 1, Wednesday).unwrap();
let datetime = date.with_hms(13, 0, 55).unwrap();
let datetime_off = datetime.assume_offset(UtcOffset::from_hms(1, 2, 3).unwrap());

println!("{date}, {datetime}, {datetime_off}");
// 2022-01-01, 2022-01-01 13:00:55.0, 2022-01-01 13:00:55.0 +01:02:03
//

// TODO fix
assert_eq!(
    datetime!(2019 - 11 - 25 15:30).saturating_add(27.hours()),
    datetime!(2019 - 11 - 26 18:30)
);
}

