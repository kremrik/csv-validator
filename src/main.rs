use csv_validator::constraints as cst;
use csv_validator::constraints::Constraint;

use csv::{
    Reader,
    StringRecord,
    StringRecordsIter,
};

use std::io;
use std::process;
use std::collections::HashMap;


fn main() {
    let mut rdr = Reader::from_reader(io::stdin());
    let header = rdr.headers().unwrap().clone();
    println!("{:?}", header);
    for result in rdr.records() {
        match result {
            Ok(record) => println!("{:?}", record),
            Err(e) => println!("{e}"),
        }
    }

    let mut constraints: HashMap<String, Box<dyn Constraint>> = HashMap::new();
    constraints.insert(String::from("foo"), Box::new(cst::IsFloat::new()));
    constraints.insert(String::from("foo"), Box::new(cst::NotEmpty::new()));
}
