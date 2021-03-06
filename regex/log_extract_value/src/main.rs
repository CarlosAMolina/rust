use std::error::Error;

use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;
use std::fs;

struct Log<'a> {
    remote_addr: &'a str,
    remote_user: &'a str,
    time_local: &'a str,
    request: &'a str,
    status: &'a str,
    body_bytes_sent: &'a str,
    http_referer: &'a str,
    http_user_agent: &'a str,
}

impl<'a> fmt::Display for Log<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{},{},{},{},{},{},{},{}",
            self.remote_addr,
            self.remote_user,
            self.time_local,
            self.request,
            self.status,
            self.body_bytes_sent,
            self.http_referer,
            self.http_user_agent,
        )
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string("application.log").expect("Something went wrong reading the file");
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r#"(?x)
          (\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}) # IPv4
          \s-\s
          (.+)                                 # Remote user
          \s\[
          (.+)                                 # Time local
          \]\s"
          (.+)                                 # Request
          "\s
          (\d{1,3})                            # Status
          \s
          (\d+)                                # Body bytes sent
          \s"
          (.+)                                 # HTTP referer
          "\s"
          (.+)                                 # HTTP user agent
          "
        "#,
        )
        .unwrap();
    }
    let mut result = Vec::<Option<String>>::new();
    for line in contents.lines() {
        println!("{}", line);

        let log = get_log(line, &RE).map(|m| m.to_string());
        if log.is_none() {
            println!("Not parsed");
        }
        result.push(log);
    }

    assert_eq!(
        vec![
            None,
            Some(
                "8.8.8.8,-,28/Oct/2021:00:18:22 +0100,GET / HTTP/1.1,200,77,-,foo bar 1"
                    .to_string()
            )
        ],
        result,
    );

    Ok(())
}

fn get_log<'a>(text: &'a str, re: &'a Regex) -> Option<Log<'a>> {
    re.captures(text).and_then(|cap| {
        let groups = (
            cap.get(1),
            cap.get(2),
            cap.get(3),
            cap.get(4),
            cap.get(5),
            cap.get(6),
            cap.get(7),
            cap.get(8),
        );
        match groups {
            (
                Some(remote_addr),
                Some(remote_user),
                Some(time_local),
                Some(request),
                Some(status),
                Some(body_bytes_sent),
                Some(http_referer),
                Some(http_user_agent),
            ) => Some(Log {
                remote_addr: remote_addr.as_str(),
                remote_user: remote_user.as_str(),
                time_local: time_local.as_str(),
                request: request.as_str(),
                status: status.as_str(),
                body_bytes_sent: body_bytes_sent.as_str(),
                http_referer: http_referer.as_str(),
                http_user_agent: http_user_agent.as_str(),
            }),
            _ => None,
        }
    })
}
