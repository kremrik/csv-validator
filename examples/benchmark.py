import csv
import json
from sys import argv, stdout


class ConstraintViolationError(Exception):
    pass


# ---------------------------------------------------------
def identity(value):
    del value
    return None


def not_empty(value):
    if value != "":
        return None
    raise ConstraintViolationError("Must be non-empty")


def is_integer(value):
    if value.isdigit():
        return None
    raise ConstraintViolationError("Must be an integer")


def is_float(value):
    try:
        float(value)
        return None
    except ValueError:
        raise ConstraintViolationError("Must be a float")


def is_number(value):
    if is_float(value):
        return None
    raise ConstraintViolationError("Must be numeric")


CONSTRAINTS = {
    "Identity": identity,
    "NotEmpty": not_empty,
    "IsInteger": is_integer,
    "IsFloat": is_float,
    "IsNumber": is_number,
}


# ---------------------------------------------------------
def get_rows(filename):
    with open(filename, "r") as f:
        reader = csv.reader(f)
        header = next(reader)
        
        for row in reader:
            yield (header, row)


def get_row_names_values(row):
    for idx, name in enumerate(row[0]):
        value = row[1][idx]
        yield (name, value)


# ---------------------------------------------------------
def get_constraints(filename):
    with open(filename, "r") as f:
        constraints = json.load(f)
    
    for col_name, constraint_names in constraints.items():
        constraints[col_name] = [
            CONSTRAINTS[constraint_name] 
            for constraint_name in constraint_names
        ]

    return constraints


def get_field_constraints(name, constraints):
    return constraints.get(name, [CONSTRAINTS["Identity"]])


# ---------------------------------------------------------
def get_row_violations(row, row_num, constraints):
    for name, value in get_row_names_values(row):
        field_constraints = get_field_constraints(
            name, constraints
        )
        for constraint in field_constraints:
            messages = []
            try:
                constraint(value)
            except ConstraintViolationError as e:
                msg = e.args[0]
                messages.append(msg)
        
        if messages:
            col_name = name
            violation = (
                row_num, 
                col_name, 
                value, 
                ", ".join(messages)
            )

            yield violation


# ---------------------------------------------------------
def main(csv_path, constraint_path):
    rows = get_rows(csv_path)
    constraints = get_constraints(constraint_path)

    writer = csv.writer(stdout)

    for row_num, row in enumerate(rows):
        violations = get_row_violations(row, row_num, constraints)
        for violation in violations:
            writer.writerow(violation)


if __name__ == "__main__":
    csv_path = argv[1]
    constraint_path = argv[2]
    main(csv_path, constraint_path)
