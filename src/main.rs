use csv_validator::constraints as cst;

use csv::{
    Reader,
    StringRecord,
    StringRecordsIter,
};

use std::io;
use std::collections::HashMap;


fn sort_constraints<'c>(
    header: &'c StringRecord, constraints: &'c HashMap<String, cst::Constraint>
) -> Vec<&'c cst::Constraint> {
    // sorts constraints by the order in which they appear in the header
    let mut output = Vec::new();

    for col in header.iter() {
        let constraint = constraints
            .get(col)
            .unwrap_or(&cst::Constraint::Identity);
        output.push(constraint);
    }

    output
}


fn bold(text: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", text)
}


fn main() {
    let constraints = HashMap::from([
        (String::from("foo"), cst::Constraint::IsInteger),
        (String::from("bar"), cst::Constraint::NotEmpty),
        (String::from("baz"), cst::Constraint::IsNumber),
    ]);

    let mut rdr = Reader::from_reader(io::stdin());
    let header = rdr.headers().unwrap().clone();

    let sorted_constraints = sort_constraints(&header, &constraints);

    for (row_num, result) in rdr.records().enumerate() {
        match result {
            Ok(record) => {
                let constraint_map = record.iter().zip(&sorted_constraints);
                for (col_num, (value, constraint)) in constraint_map.enumerate() {
                    match cst::check(&value, &constraint) {
                        Ok(_) => continue,
                        Err(e) => {
                            let rnum = bold(&format!("{row_num}"));
                            let name = bold(&header[col_num]);
                            let valu = bold(value);
                            eprintln!("row=[{rnum}], col=[{name}], value=[{valu}], errors=[{e}]");
                        },
                    }
                }
            },
            Err(e) => println!("{e}"),
        }
    }
}
