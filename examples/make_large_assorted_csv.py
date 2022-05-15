import csv


COLS = ["foo", "bar", "baz"]

BASE = [
    ("hi", 2, None),
    (4, None, 6),
    (7, 8, 9)
]

REPEAT = 500_000

FILENAME = "large_assortment.csv"


with open(FILENAME, "w") as f:
    writer = csv.writer(f)
    writer.writerow(COLS)

    for _ in range(REPEAT):
        for row in BASE:
            writer.writerow(row)
