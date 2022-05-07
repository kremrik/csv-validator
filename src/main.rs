use csv_validator::constraints;

use csv::{
    Reader,
    StringRecord,
    StringRecordsIter,
};

use std::io;
use std::process;


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
}
