import csv
from sys import argv


COLS = ["foo", "bar", "baz"]

BASE = [
    ("hi", 2, None),
    (4, None, 6),
    (7, 8, 9)
]


repeat_size = int(argv[1])
filename = "assortment_{}.csv".format(repeat_size)


with open(filename, "w") as f:
    writer = csv.writer(f)
    writer.writerow(COLS)

    for _ in range(repeat_size):
        for row in BASE:
            writer.writerow(row)
