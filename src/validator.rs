use crate::constraints as cst;
use csv::{
    StringRecord,
};


#[derive(Debug, PartialEq)]
pub struct ConstraintViolation<'cv> {
    pub name: &'cv str,
    pub value: &'cv str,
    pub message: Vec<String>,
}


pub fn validate_record<'v>(
    record: &'v StringRecord,
    header: &'v StringRecord,
    constraints: &'v Vec<cst::Constraint>,
) -> Option<ConstraintViolation<'v>> {
    let constraint_map = record.iter().zip(constraints);
    for (col_num, (value, constraint)) in constraint_map.enumerate() {
        println!("FIELD");
        match cst::check(&value, &constraint) {
            Ok(_) => continue,
            Err(e) => {
                let violation = ConstraintViolation {
                    name: &header[col_num],
                    value: value,
                    message: vec![e],
                };
                return Some(violation);
            },
        }
    }

    return None
}
