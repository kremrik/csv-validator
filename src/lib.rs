pub mod constraints;
pub mod parser;
pub mod validator;

use constraints as cst;

use csv::{Reader, StringRecord, StringRecordsIter};

use std::io;


pub struct CsvValidator<'r, R> {
    records: StringRecordsIter<'r, R>, 
    constraints: &'r Vec<Vec<cst::Constraint>>,
    header: StringRecord,
    current_row: usize,
    violations: Vec<validator::ConstraintViolation<'r>>
}

impl<'r, R> CsvValidator<'r, R>
where R: io::Read {
    pub fn new(
        reader: &'r Reader<R>,
        constraints: &'r Vec<Vec<cst::Constraint>>
    ) -> CsvValidator<'r, R> {
        let header = reader.headers().unwrap().clone();
        let current_row: usize = 0;

        CsvValidator {
            records: reader.records(),
            violations: Vec::new(),
            constraints,
            header,
            current_row,
        }
    }

    fn get_next_violations(&mut self) -> Vec<validator::ConstraintViolation<'r>> {
        loop {
            let result = self.records.next();
            if result.is_none() {
                break
            }

            let record = result.unwrap().unwrap();
            
            match validator::validate_record(
                self.current_row, 
                &record, 
                &self.header, 
                &self.constraints) {
                None => {
                    self.current_row += 1;
                    continue
                },
                Some(violations) => {
                    self.current_row += 1;
                    return violations
                }
            }
        }

        return Vec::new()
    }
}

impl<'r, R> Iterator for CsvValidator<'r, R>
where R: io::Read {
    type Item = validator::ConstraintViolation<'r>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.records.next();
        if result.is_none() {
            return None
        }

        match result.unwrap() {
            Ok(record) => {
                match validator::validate_record(
                    self.current_row, 
                    &record, 
                    &self.header, 
                    &self.constraints
                ) {
                    None => return None,
                    Some(violations) => {
                        let violation = violations[0];
                        return Some(violation);
                    }
                }
            }
            Err(e) => println!("{e}"),
        }
    }
}
