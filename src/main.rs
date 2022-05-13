use csv_validator::constraints as cst;
use csv_validator::validator;

use csv::{Reader, StringRecord};

use std::collections::HashMap;
use std::io;

fn sort_constraints<'c>(
    header: &'c StringRecord,
    constraints: &'c HashMap<String, Vec<cst::Constraint>>,
) -> Vec<Vec<cst::Constraint>> {
    // sorts constraints by the order in which they appear in the header
    let mut output = Vec::new();
    let default = vec![cst::Constraint::Identity];

    for col in header.iter() {
        let constraint = constraints.get(col).unwrap_or(&default);
        output.push(constraint.into_iter().cloned().collect());
    }

    output
}

fn main() {
    let constraints = HashMap::from([
        (String::from("bar"), vec![cst::Constraint::NotEmpty]),
        (
            String::from("baz"),
            vec![cst::Constraint::IsNumber, cst::Constraint::NotEmpty],
        ),
    ]);

    let mut rdr = Reader::from_reader(io::stdin());
    let header = rdr.headers().unwrap().clone();

    let sorted_constraints = sort_constraints(&header, &constraints);

    for (row_num, result) in rdr.records().enumerate() {
        match result {
            Ok(record) => match validator::validate_record(&record, &header, &sorted_constraints) {
                None => continue,
                Some(violations) => {
                    let rnum = &format!("{row_num}");
                    for violation in violations {
                        eprintln!("row {rnum}: {violation}");
                    }
                }
            },
            Err(e) => println!("{e}"),
        }
    }
}
