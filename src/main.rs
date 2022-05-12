use csv_validator::constraints as cst;
use csv_validator::validator;

use csv::{
    Reader,
    StringRecord,
};

use std::io;
use std::collections::HashMap;


fn sort_constraints<'c>(
    header: &'c StringRecord, constraints: &'c HashMap<String, cst::Constraint>
) -> Vec<cst::Constraint> {
    // sorts constraints by the order in which they appear in the header
    let mut output = Vec::new();

    for col in header.iter() {
        let constraint = constraints
            .get(col)
            .unwrap_or(&cst::Constraint::Identity);
        output.push(*constraint);
    }

    output
}


fn main() {
    let constraints = HashMap::from([
        (String::from("bar"), cst::Constraint::NotEmpty),
        (String::from("baz"), cst::Constraint::IsNumber),
    ]);

    let mut rdr = Reader::from_reader(io::stdin());
    let header = rdr.headers().unwrap().clone();

    let sorted_constraints = sort_constraints(&header, &constraints);

    for (row_num, result) in rdr.records().enumerate() {
        match result {
            Ok(record) => {
                match validator::validate_record(&record, &header, &sorted_constraints) {
                    None => continue,
                    Some(violations) => {
                        let rnum = &format!("{row_num}");
                        for violation in violations {
                            eprintln!("row {rnum}: {violation}");
                        }
                    }
                }
            },
            Err(e) => println!("{e}"),
        }
    }
}
