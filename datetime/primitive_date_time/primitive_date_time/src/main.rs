use time::ext::NumericalDuration;
use time::format_description;
use time::macros::datetime;

fn main() {
    assert_eq!(
        datetime!(2019 - 11 - 25 15:30).saturating_add(27.hours()),
        datetime!(2019 - 11 - 26 18:30)
    );
    assert_eq!(
        datetime!(2019 - 11 - 26 18:30:59).saturating_add(1.seconds()),
        datetime!(2019 - 11 - 26 18:31:00),
    );
    test_format_description();
}

fn test_format_description() {
    let format =
        format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
    assert_eq!(
        datetime!(2020-01-02 03:04:05).format(&format).unwrap(),
        "2020-01-02 03:04:05"
    );
}
