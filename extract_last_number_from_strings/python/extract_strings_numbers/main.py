from timeit import default_timer as timer
from typing import List
import re


def run():
    # The first time a regex is called it takes more time.
    run_parse(get_filenames_numbers_with_regex_captures, "regex captures 1st time")
    # Call the function again without the time required by a regex when it is called the first time
    run_parse(get_filenames_numbers_with_regex_captures, "regex captures")
    run_parse(get_filenames_numbers_with_regex_search, "regex search")
    run_parse(get_filenames_numbers_with_split, "split")


def run_parse(parse_function, parse_description: str):
    filenames = [
        "foo",
        "foo.txt",
        "access.log",
        "access.log.5",
        "access.log.2",
        "access.log.10",
        "access.log.1",
    ]
    # loops_number = 5_000
    loops_number = 1
    start = timer()
    for _ in range(loops_number):
        result = parse_function(filenames)
        # print(result)
    end = timer()
    duration = end - start
    print(f"Time elapsed with {parse_description}: {duration * 1000}ms")
    assert result == [5, 2, 10, 1]


def get_filenames_numbers_with_regex_search(filenames: List[str]) -> List[int]:
    result = []
    for filename in filenames:
        re_result = re.search("^access\.log\.(?P<file_number>\d+)", filename)
        if re_result is not None:
            number = int(re_result.group("file_number"))
            result.append(number)
    return result


def get_filenames_numbers_with_regex_captures(filenames: List[str]) -> List[int]:
    result = []
    for filename in filenames:
        re_result = re.match("^access\.log\.(?P<file_number>\d+)", filename)
        if re_result is not None:
            number = int(re_result.group("file_number"))
            result.append(number)
    return result


def get_filenames_numbers_with_split(filenames: List[str]) -> List[int]:
    result = []
    for filename in filenames:
        last_part = filename.split(".")[-1]
        try:
            number = int(last_part)
            result.append(number)
        except ValueError:
            pass
    return result


if __name__ == "__main__":
    run()
