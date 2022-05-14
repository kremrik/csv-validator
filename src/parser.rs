use crate::constraints::Constraint;

use serde_json;

use std::collections::HashMap;

type ConstraintFile = HashMap<String, Vec<Constraint>>;


pub fn get_constraint_map(json_text: &str) -> ConstraintFile {
    let constraint_map: ConstraintFile = serde_json::from_str(json_text).unwrap();
    constraint_map
}


// TESTS
// --------------------------------------------------------
#[cfg(test)]
mod test {
    use super::{
        Constraint,
        ConstraintFile,
        get_constraint_map,
    };

    #[test]
    fn test_no_constraints() {
        let json_text = "{}";
        let expect = ConstraintFile::new();
        let actual = get_constraint_map(json_text);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_mult_valid_constraints_mult_fields() {
        let json_text = r#"
            {
                "foo": ["NotEmpty", "IsNumber"],
                "bar": ["NotEmpty", "IsFloat"],
                "baz": ["NotEmpty"]
            }
        "#;

        let mut expect = ConstraintFile::new();
        expect.insert(String::from("foo"), vec![Constraint::NotEmpty, Constraint::IsNumber]);
        expect.insert(String::from("bar"), vec![Constraint::NotEmpty, Constraint::IsFloat]);
        expect.insert(String::from("baz"), vec![Constraint::NotEmpty]);

        let actual = get_constraint_map(json_text);
        assert_eq!(expect, actual);
    }
}
