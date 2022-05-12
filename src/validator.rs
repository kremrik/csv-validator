use crate::constraints as cst;
use csv::{
    StringRecord,
};

use std::fmt;


#[derive(Debug, PartialEq)]
pub struct ConstraintViolation<'cv> {
    pub name: &'cv str,
    pub value: &'cv str,
    pub message: Vec<String>,
}

impl<'cv> fmt::Display for ConstraintViolation<'cv> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "col=[{}], value=[{}], errors={:?}", 
            self.name, self.value, self.message
        )
    }
}


pub fn validate_record<'v>(
    record: &'v StringRecord,
    header: &'v StringRecord,
    constraints: &'v Vec<cst::Constraint>,
) -> Option<Vec<ConstraintViolation<'v>>> {
    let constraint_map = record.iter().zip(constraints);
    let mut violations = Vec::new();

    for (col_num, (value, constraint)) in constraint_map.enumerate() {
        match cst::check(&value, &constraint) {
            Ok(_) => continue,
            Err(e) => {
                let violation = ConstraintViolation {
                    name: &header[col_num],
                    value: value,
                    message: vec![e],
                };
                violations.push(violation);
            },
        }
    }

    if !violations.is_empty() {
        return Some(violations)
    } else {
        return None
    }
}
