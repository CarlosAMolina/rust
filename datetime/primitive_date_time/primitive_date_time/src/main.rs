use time::ext::NumericalDuration;
use time::macros::datetime;

fn main() {
    assert_eq!(
        datetime!(2019 - 11 - 26 18:30),
        datetime!(2019 - 11 - 25 15:30).saturating_add(27.hours())
    );
}
