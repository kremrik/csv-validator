use csv_validator::cli;
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

fn execute(args: cli::Args) {
    let constraint_file = args.constraints;
    let json_text = fs::read_to_string(constraint_file).unwrap();
    let constraints = parser::get_constraint_map(&json_text);

    let mut rdr = Reader::from_reader(io::stdin());
    let mut wtr = Writer::from_writer(io::stdout());

    let header = rdr.headers().unwrap().clone();

    let sorted_constraints = sort_constraints(&header, &constraints);
    let records = rdr.records();

    for (row_num, result) in records.enumerate() {
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
    let args = cli::parse_args();
    execute(args)
}
