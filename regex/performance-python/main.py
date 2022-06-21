from timeit import default_timer as timer
from typing import List, Optional
import re

# https://stackoverflow.com/questions/7370801/how-to-measure-elapsed-time-in-python#7370824


class Log:

    def __init__(
        self,
        remote_addr: str,
        remote_user: str,
        time_local: str,
        request: str,
        status: str,
        body_bytes_sent: str,
        http_referer: str,
        http_user_agent: str,
    ):
        self.remote_addr = remote_addr
        self.remote_user = remote_user
        self.time_local = time_local
        self.request = request
        self.status = status
        self.body_bytes_sent = body_bytes_sent
        self.http_referer = http_referer
        self.http_user_agent = http_user_agent

    def __repr__(self):
        return "{},{},{},{},{},{},{},{}".format(
            self.remote_addr,
            self.remote_user,
            self.time_local,
            self.request,
            self.status,
            self.body_bytes_sent,
            self.http_referer,
            self.http_user_agent,
        )

# https://docs.nginx.com/nginx/admin-guide/monitoring/logging/
REGEX = re.compile(
    r"""
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
""",
    re.VERBOSE,
)


def get_log(line: str) -> Optional[Log]:
    match = re.search(REGEX, line)
    return (
        None
        if match is None
        else Log(
            remote_addr=match.group(1),
            remote_user=match.group(3),
            time_local=match.group(4),
            request=match.group(5),
            status=match.group(6),
            body_bytes_sent=match.group(7),
            http_referer=match.group(8),
            http_user_agent=match.group(9),
        )
    )

def get_result_without_regex_one_loop(text: str) -> Log:
    log_parts_index = [0]
    characters_to_match = [
        ' ',
        ' ',
        '[',
        ']',
        '"',
        '"',
        ' ',
        ' ',
        '"',
        '"',
        '"',
    ]
    match_index = 0
    text_index = 0
    for item in text:
        if item == characters_to_match[match_index]:
            log_parts_index.append(text_index)
            if match_index < len(characters_to_match) - 1:
                match_index += 1
        text_index += 1
    return Log(
        remote_addr= text[log_parts_index[0]:log_parts_index[1]],
        remote_user= text[log_parts_index[2] + 1:log_parts_index[3] - 1],
        time_local= text[log_parts_index[3] + 1:log_parts_index[4]],
        request= text[log_parts_index[5] + 1:log_parts_index[6]],
        status= text[log_parts_index[7] + 1: log_parts_index[8]],
        body_bytes_sent= text[log_parts_index[8] + 1: log_parts_index[9] - 1],
        http_referer= text[log_parts_index[9] + 1:log_parts_index[10]],
        http_user_agent= text[log_parts_index[11] + 1:log_parts_index[12]],
    )


def run():
    log = (
        '8.8.8.8 - abc [28/Nov/2021:00:18:22 +0100] "GET / HTTP/1.1" 200 77 "-"'
        ' "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36'
        ' (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36"'
    )
    loops_number = 5_000
    start = timer()
    for i in range(loops_number):
        result = get_log(log)
        #print(result)
    end = timer()
    duration = end - start
    #print(f"Time elapsed: {duration}s")
    print(f"Time elapsed groups: {duration * 1000}ms")
    start = timer()
    for i in range(loops_number):
        result = get_result_without_regex_one_loop(log)
        #print(result)
    end = timer()
    duration = end - start
    #print(f"Time elapsed: {duration}s")
    print(f"Time elapsed without regex: {duration * 1000}ms")





if __name__ == "__main__":
    run()
