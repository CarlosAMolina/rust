use std::fmt;
use std::time::Instant;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let log = r#"8.8.8.8 - - [28/Oct/2021:00:18:22 +0100] "GET / HTTP/1.1" 200 77 "-" "foo bar 1""#;
    println!("Parsing log: {:?}", log);
    let loops_number = 5_000;
    //let loops_number = 1;
    let start = Instant::now();
    for _ in 0..loops_number {
        let _result = get_result_with_regex_match(log);
        //println!("{:?}", _result);
    }
    let duration_match = start.elapsed();
    let start = Instant::now();
    for _ in 0..loops_number {
        let _result = get_result_with_regex_find(log);
        //println!("{:?}", _result);
    }
    let duration_find = start.elapsed();
    let start = Instant::now();
    for _ in 0..loops_number {
        let _result = get_result_with_regex_groups(log);
        //println!("{:?}", _result);
    }
    let duration_groups = start.elapsed();
    let start = Instant::now();
    for _ in 0..loops_number {
        let _result = get_result_without_regex(log);
        //println!("{:?}", _result);
    }
    let duration_without_regex = start.elapsed();
    let start = Instant::now();
    for _ in 0..loops_number {
        let _result = get_result_without_regex_one_loop(log);
        //println!("{:?}", _result);
    }
    let duration_without_regex_one_loop = start.elapsed();
    println!("Time elapsed match: {:?}", duration_match);
    println!("Time elapsed find: {:?}", duration_find);
    println!("Time elapsed groups: {:?}", duration_groups);
    println!("Time elapsed without regex: {:?}", duration_without_regex);
    println!(
        "Time elapsed without regex one loop: {:?}",
        duration_without_regex_one_loop
    );
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

fn get_result_with_regex_match(text: &str) -> Option<Log> {
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

fn get_result_with_regex_find(text: &str) -> Option<Log> {
    let mut characters_checked = 0;
    lazy_static! {
        static ref RE_IPV4: Regex = Regex::new(r"(\d{1,3}[\.]){3}\d{1,3}",).unwrap();
    }
    let mut log_parts_index = vec![0];
    let mut re_result = RE_IPV4.find(text).unwrap();
    log_parts_index.push(re_result.end());
    characters_checked += re_result.end();
    characters_checked += 3;
    log_parts_index.push(characters_checked);
    lazy_static! {
        static ref RE_REMOTE_USER: Regex = Regex::new(r".+\s\[").unwrap();
    }
    re_result = RE_REMOTE_USER.find(&text[characters_checked..]).unwrap();
    characters_checked += re_result.end();
    log_parts_index.push(characters_checked - 2);
    lazy_static! {
        static ref RE_TIME_LOCAL: Regex =
            Regex::new(r"\d{2}/[[:alpha:]]{3}/\d{4}:\d{2}:\d{2}:\d{2}\s\+\d{4}",).unwrap();
    }
    log_parts_index.push(characters_checked);
    re_result = RE_TIME_LOCAL.find(&text[characters_checked..]).unwrap();
    characters_checked += re_result.end();
    log_parts_index.push(characters_checked);
    characters_checked += 3;
    log_parts_index.push(characters_checked);
    lazy_static! {
        static ref RE_REQUEST: Regex = Regex::new(r#"[[:alpha:]].*"\s\d"#).unwrap();
    }
    re_result = RE_REQUEST.find(&text[characters_checked..]).unwrap();
    characters_checked += re_result.end() - 1;
    log_parts_index.push(characters_checked - 2);
    log_parts_index.push(characters_checked);
    //lazy_static! {
    //    static ref RE_STATUS: Regex = Regex::new(r"\d{3}").unwrap();
    //}
    //let status = RE_STATUS.find(&text[characters_checked..]).unwrap();
    //characters_checked += status.end();
    characters_checked += 3;
    log_parts_index.push(characters_checked);
    log_parts_index.push(characters_checked + 1);
    lazy_static! {
        static ref RE_BODY_BYTES_SENT: Regex = Regex::new(r"\d{1,3}").unwrap();
    }
    let body_bytes_sent = RE_BODY_BYTES_SENT
        .find(&text[characters_checked..])
        .unwrap();
    characters_checked += body_bytes_sent.end();
    log_parts_index.push(characters_checked);
    characters_checked += 2;
    log_parts_index.push(characters_checked);
    lazy_static! {
        static ref RE_HTTP_REFERER: Regex = Regex::new(r#".*"\s"#).unwrap();
    }
    let http_referer = RE_HTTP_REFERER.find(&text[characters_checked..]).unwrap();
    characters_checked += http_referer.end();
    log_parts_index.push(characters_checked - 2);
    characters_checked += 1;
    //lazy_static! {
    //    static ref RE_HTTP_USER_AGENT: Regex = Regex::new(r#".*"$"#).unwrap();
    //}
    //let http_user_agent = RE_HTTP_USER_AGENT.find(&text[characters_checked..]).unwrap();
    log_parts_index.push(characters_checked);
    log_parts_index.push(text.len() - 1);
    Some(Log {
        remote_addr: &text[log_parts_index[0]..log_parts_index[1]],
        remote_user: &text[log_parts_index[2]..log_parts_index[3]],
        time_local: &text[log_parts_index[4]..log_parts_index[5]],
        request: &text[log_parts_index[6]..log_parts_index[7]],
        status: &text[log_parts_index[8]..log_parts_index[9]],
        body_bytes_sent: &text[log_parts_index[10]..log_parts_index[11]],
        http_referer: &text[log_parts_index[12]..log_parts_index[13]],
        http_user_agent: &text[log_parts_index[14]..log_parts_index[15]],
    })
}

fn get_result_with_regex_groups(text: &str) -> Option<Log> {
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

fn get_match_len(bytes: &[u8], byte_to_match: u8) -> usize {
    for (i, &item) in bytes.iter().enumerate() {
        if item == byte_to_match {
            return i;
        }
    }
    bytes.len()
}

fn get_result_without_regex(text: &str) -> Option<Log> {
    let mut log_parts_index = vec![0];
    let bytes = text.as_bytes();
    // remote_addr
    log_parts_index.push(get_match_len(&bytes, b' '));
    // remote_user
    log_parts_index.push(log_parts_index[log_parts_index.len() - 1] + 3);
    log_parts_index.push(
        log_parts_index[log_parts_index.len() - 1]
            + get_match_len(&bytes[log_parts_index[log_parts_index.len() - 1]..], b'[')
            - 1,
    );
    // time_local
    log_parts_index.push(log_parts_index[log_parts_index.len() - 1] + 2);
    log_parts_index.push(
        log_parts_index[log_parts_index.len() - 1]
            + get_match_len(&bytes[log_parts_index[log_parts_index.len() - 1]..], b']'),
    );
    // request
    log_parts_index.push(log_parts_index[log_parts_index.len() - 1] + 3);
    log_parts_index.push(
        log_parts_index[log_parts_index.len() - 1]
            + get_match_len(&bytes[log_parts_index[log_parts_index.len() - 1]..], b'"'),
    );
    // status
    log_parts_index.push(log_parts_index[log_parts_index.len() - 1] + 2);
    log_parts_index.push(
        log_parts_index[log_parts_index.len() - 1]
            + get_match_len(&bytes[log_parts_index[log_parts_index.len() - 1]..], b' '),
    );
    // body_bytes_sent
    log_parts_index.push(log_parts_index[log_parts_index.len() - 1] + 1);
    log_parts_index.push(
        log_parts_index[log_parts_index.len() - 1]
            + get_match_len(&bytes[log_parts_index[log_parts_index.len() - 1]..], b' '),
    );
    // http_referer
    log_parts_index.push(log_parts_index[log_parts_index.len() - 1] + 2);
    log_parts_index.push(
        log_parts_index[log_parts_index.len() - 1]
            + get_match_len(&bytes[log_parts_index[log_parts_index.len() - 1]..], b'"'),
    );
    // http_user_agent
    log_parts_index.push(log_parts_index[log_parts_index.len() - 1] + 3);
    log_parts_index.push(text.len() - 1);
    //println!("{:?}", &log_parts_index);
    //let log_parts_index = vec![0, 7, 10, 11, 13, 39, 42, 56, 58, 61, 62, 64, 66, 67, 70, 79];
    Some(Log {
        remote_addr: &text[log_parts_index[0]..log_parts_index[1]],
        remote_user: &text[log_parts_index[2]..log_parts_index[3]],
        time_local: &text[log_parts_index[4]..log_parts_index[5]],
        request: &text[log_parts_index[6]..log_parts_index[7]],
        status: &text[log_parts_index[8]..log_parts_index[9]],
        body_bytes_sent: &text[log_parts_index[10]..log_parts_index[11]],
        http_referer: &text[log_parts_index[12]..log_parts_index[13]],
        http_user_agent: &text[log_parts_index[14]..log_parts_index[15]],
    })
}

fn get_result_without_regex_one_loop(text: &str) -> Option<Log> {
    let mut log_parts_index = vec![0];
    let characters_to_match = vec![
        b' ', b' ', b'[', b']', b'"', b'"', b' ', b' ', b'"', b'"', b'"',
    ];
    let bytes = text.as_bytes();
    let mut match_index = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == characters_to_match[match_index] {
            log_parts_index.push(i);
            if match_index < characters_to_match.len() - 1 {
                match_index += 1;
            }
        }
    }
    if log_parts_index.len() == 13 {
        Some(Log {
            remote_addr: &text[log_parts_index[0]..log_parts_index[1]],
            remote_user: &text[log_parts_index[2] + 1..log_parts_index[3] - 1],
            time_local: &text[log_parts_index[3] + 1..log_parts_index[4]],
            request: &text[log_parts_index[5] + 1..log_parts_index[6]],
            status: &text[log_parts_index[7] + 1..log_parts_index[8]],
            body_bytes_sent: &text[log_parts_index[8] + 1..log_parts_index[9] - 1],
            http_referer: &text[log_parts_index[9] + 1..log_parts_index[10]],
            http_user_agent: &text[log_parts_index[11] + 1..log_parts_index[12]],
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_result_without_regex_one_loop_with_correct_log() {
        assert_ne!(
            None,
            get_result_without_regex_one_loop(
                r#"8.8.8.8 - - [28/Oct/2021:00:18:22 +0100] "GET / HTTP/1.1" 200 77 "-" "foo bar 1""#
            )
        );
    }

    #[test]
    fn test_get_result_without_regex_one_loop_with_correct_log_and_different_remote_user() {
        let log_parsed = get_result_without_regex_one_loop(
                r#"8.8.8.8 - foo user [28/Oct/2021:00:18:22 +0100] "GET / HTTP/1.1" 200 77 "-" "foo bar 1""#
        ).unwrap();
        assert_eq!("foo user", log_parsed.remote_user);
    }

    #[test]
    fn test_get_result_without_regex_one_loop_with_incorrect_log() {
        assert_eq!(
            None,
            get_result_without_regex_one_loop(
                r#"8.8.8.8 - - [28/Oct/2021:00:18:22 +0100 "GET / HTTP/1.1" 200 77 "-" "foo bar 1""#
            )
        );
    }
}
