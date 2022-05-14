use csv_validator::constraints as cst;
use csv_validator::parser;
use csv_validator::validator;

use csv::{Reader, StringRecord, Writer};

use std::collections::HashMap;
use std::fs;
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

fn cli() {
    let constraints = HashMap::from([
        (String::from("bar"), vec![cst::Constraint::NotEmpty]),
        (
            String::from("baz"),
            vec![cst::Constraint::IsNumber, cst::Constraint::NotEmpty],
        ),
    ]);

    let mut rdr = Reader::from_reader(io::stdin());
    let mut wtr = Writer::from_writer(io::stdout());

    let header = rdr.headers().unwrap().clone();

    let sorted_constraints = sort_constraints(&header, &constraints);

    for (row_num, result) in rdr.records().enumerate() {
        match result {
            Ok(record) => {
                match validator::validate_record(row_num, &record, &header, &sorted_constraints) {
                    None => continue,
                    Some(violations) => {
                        for violation in violations {
                            wtr.serialize(violation).unwrap();
                            wtr.flush().unwrap(); // TODO: buffer batches instead of one at a time
                        }
                    }
                }
            }
            Err(e) => println!("{e}"),
        }
    }
}

fn main() {
    // cli()

    let filename = "constraints.json";
    let json_text = fs::read_to_string(filename).unwrap();
    let constraint_map = parser::get_constraint_map(&json_text);
    println!("{:?}", constraint_map);
}
