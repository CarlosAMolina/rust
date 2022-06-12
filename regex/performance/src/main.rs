use std::fmt;
use std::time::Instant;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let log =
        "8.8.8.8 - - [28/Oct/2021:00:18:22 +0100] \"GET / HTTP/1.1\" 200 77 \"-\" \"foo bar 1\"";
    let loops_number = 5_000;
    let loops_number = 1;
    let start = Instant::now();
    for _ in 0..loops_number {
        let _result = get_regex_result_with_match(&log);
    }
    let duration_match = start.elapsed();
    let start = Instant::now();
    for _ in 0..loops_number {
        let result = get_regex_result_with_find(&log);
        println!("{:?}", result);
    }
    let duration_find = start.elapsed();
    let start = Instant::now();
    for _ in 0..loops_number {
        let result = get_regex_result_with_groups(&log);
        //println!("{:?}", result);
    }
    let duration_groups = start.elapsed();
    println!("Time elapsed macth: {:?}", duration_match);
    println!("Time elapsed find: {:?}", duration_find);
    println!("Time elapsed groups: {:?}", duration_groups);
}

#[derive(Debug, PartialEq)]
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

fn get_regex_result_with_match(text: &str) -> Option<Log> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r#"(?x)
          ^
          ((\d{1,3}[\.]){3}\d{1,3}) # IPv4
          \s-\s
          (.+)                      # Remote user
          \s\[
          (.+)                      # Time local
          \]\s"
          (.*)                      # Request
          "\s
          (\d{1,3})                 # Status
          \s
          (\d+)                     # Body bytes sent
          \s"
          (.+)                      # HTTP referer
          "\s"
          (.+)                      # HTTP user agent
          "
          $
        "#,
        )
        .unwrap();
    }
    if RE.is_match(text) {
        return Some(Log {
            remote_addr: "remote_addr",
            remote_user: "remote_user",
            time_local: "time_local",
            request: "request",
            status: "status",
            body_bytes_sent: "body_bytes_sent",
            http_referer: "http_referer",
            http_user_agent: "http_user_agent",
        });
    }
    None
}

fn get_regex_result_with_find(text: &str) -> Option<Log> {
    lazy_static! {
        static ref RE_IPV4: Regex = Regex::new(r"(\d{1,3}[\.]){3}\d{1,3}",).unwrap();
    }

    let remote_addr = RE_IPV4.find(text).unwrap();

    lazy_static! {
        static ref RE_REMOTE_USER: Regex = Regex::new(r"(.+)\s\[").unwrap();
    }
    //println!("{:?}", &text[remote_addr.start()..remote_addr.end()]);
    let remote_user = RE_REMOTE_USER.find(&text[remote_addr.end() + 3..]).unwrap();
    lazy_static! {
        static ref RE_TIME_LOCAL: Regex = Regex::new(r"\[.+\]",).unwrap();
    }

    let time_local = RE_TIME_LOCAL.find(text).unwrap();
    Some(Log {
        remote_addr: remote_addr.as_str(),
        remote_user: &remote_user.as_str()[..remote_user.as_str().len() - 2],
        time_local: &text[time_local.start()+1..time_local.end() -1],
        request: "request",
        status: "status",
        body_bytes_sent: "body_bytes_sent",
        http_referer: "http_referer",
        http_user_agent: "http_user_agent",
    })
}

fn get_regex_result_with_groups(text: &str) -> Option<Log> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r#"(?x)
          ^
          ((\d{1,3}[\.]){3}\d{1,3}) # IPv4
          \s-\s
          (.+)                      # Remote user
          \s\[
          (.+)                      # Time local
          \]\s"
          (.*)                      # Request
          "\s
          (\d{1,3})                 # Status
          \s
          (\d+)                     # Body bytes sent
          \s"
          (.+)                      # HTTP referer
          "\s"
          (.+)                      # HTTP user agent
          "
          $
        "#,
        )
        .unwrap();
    }
    RE.captures(text).and_then(|cap| {
        let groups = (
            cap.get(1),
            cap.get(3),
            cap.get(4),
            cap.get(5),
            cap.get(6),
            cap.get(7),
            cap.get(8),
            cap.get(9),
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
