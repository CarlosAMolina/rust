use std::error::Error;

use regex::Regex;
use std::fmt;

struct Log<'a> {
    remote_addr: &'a str,
    remote_user: &'a str,
    time_local: &'a str,
}

impl<'a> fmt::Display for Log<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{},{},{}",
            self.remote_addr, self.remote_user, self.time_local
        )
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let text = r#"
    1.1.1.1 foo
    8.8.8.8 - - [28/Oct/2021:00:18:22 +0100] "GET / HTTP/1.1" 200 77 "-" "foo bar 1"
    150.10.100.23 - foo_user [10/Oct/2022:05:18:22 +0100] "GET / HTTP/1.1" 200 77 "-" "foo bar 1"#;

    let re = Regex::new(
        r#"(?x)
          (\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}) # IPv4
          \s-\s
          (.+)                                 # Remote user
          \s\[
          (.*)                                 # Time local
          \]
          .*
          "#,
    )?;

    let logs = re.captures_iter(text).filter_map(|cap| {
        let groups = (cap.get(1), cap.get(2), cap.get(3));
        match groups {
            (Some(remote_addr), Some(remote_user), Some(time_local)) => Some(Log {
                remote_addr: remote_addr.as_str(),
                remote_user: remote_user.as_str(),
                time_local: time_local.as_str(),
            }),
            _ => None,
        }
    });

    assert_eq!(
        vec![
            "8.8.8.8,-,28/Oct/2021:00:18:22 +0100",
            "150.10.100.23,foo_user,10/Oct/2022:05:18:22 +0100",
        ],
        logs.map(|m| m.to_string()).collect::<Vec<_>>(),
    );

    Ok(())
}
