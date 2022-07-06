# https://stackoverflow.com/questions/66250215/rust-and-gzipped-files

from timeit import default_timer as timer
import gzip


def process_file(path):
    with gzip.open(path) as fp:
        count = 0
        for line in fp:
            #print(line)
            count += 1
    print(f"Found {count} events")


if __name__ == "__main__":
    path = "access.log.10.gz";
    start = timer()
    process_file(path)
    print(
        "Time elapsed: {duration}ms".format(
            duration = (timer() - start) * 1000
        )
    )
