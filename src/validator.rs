use crate::constraints as cst;
use csv::StringRecord;

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
    constraints: &'v Vec<Vec<cst::Constraint>>,
) -> Option<Vec<ConstraintViolation<'v>>> {
    let constraint_map = record.iter().zip(constraints);
    let mut violations = Vec::new();

    for (col_num, (value, constraints)) in constraint_map.enumerate() {
        let mut errors = Vec::new();

        for constraint in constraints {
            match cst::check(&value, &constraint) {
                Ok(_) => continue,
                Err(e) => errors.push(e),
            }
        }
        
        if !errors.is_empty() {
            let violation = ConstraintViolation {
                name: &header[col_num],
                value: value,
                message: errors,
            };
            violations.push(violation);
        }
    }

    if !violations.is_empty() {
        return Some(violations);
    } else {
        return None;
    }
}

// TESTS
// --------------------------------------------------------
#[cfg(test)]
mod test {
    use super::{cst, validate_record, ConstraintViolation, StringRecord};

    #[test]
    fn test_no_constraints() {
        let record = StringRecord::from(vec!["1", "2", "3"]);
        let header = StringRecord::from(vec!["foo", "bar", "baz"]);
        let constraints: Vec<Vec<cst::Constraint>> = Vec::new();
        let actual = validate_record(&record, &header, &constraints);
        assert!(actual.is_none());
    }

    #[test]
    fn test_no_violations() {
        let record = StringRecord::from(vec!["1", "2", "3"]);
        let header = StringRecord::from(vec!["foo", "bar", "baz"]);
        let constraints = vec![
            vec![cst::Constraint::Identity],
            vec![cst::Constraint::NotEmpty],
            vec![cst::Constraint::IsNumber],
        ];
        let actual = validate_record(&record, &header, &constraints);
        assert!(actual.is_none());
    }

    #[test]
    fn test_one_violation() {
        let record = StringRecord::from(vec!["1", "2", "hi"]);
        let header = StringRecord::from(vec!["foo", "bar", "baz"]);
        let constraints = vec![
            vec![cst::Constraint::Identity],
            vec![cst::Constraint::NotEmpty],
            vec![cst::Constraint::IsNumber],
        ];
        let expect = Some(vec![ConstraintViolation {
            message: vec![String::from("Must be numeric")],
            name: "baz",
            value: "hi",
        }]);
        let actual = validate_record(&record, &header, &constraints);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_mult_violations_for_field() {
        let record = StringRecord::from(vec!["1", "2", ""]);
        let header = StringRecord::from(vec!["foo", "bar", "baz"]);
        let constraints = vec![
            vec![cst::Constraint::Identity],
            vec![cst::Constraint::Identity],
            vec![cst::Constraint::IsNumber, cst::Constraint::NotEmpty],
        ];
        let expect = Some(vec![ConstraintViolation {
            message: vec![String::from("Must be numeric"), String::from("Must be non-empty")],
            name: "baz",
            value: "",
        }]);
        let actual = validate_record(&record, &header, &constraints);
        assert_eq!(expect, actual);
    }
}
