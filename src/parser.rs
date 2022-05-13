use crate::constraints::Constraint;

use serde_json;

use std::collections::HashMap;

type ConstraintFile = HashMap<String, Vec<Constraint>>;


pub fn get_constraint_map(json_text: &str) -> ConstraintFile {
    let constraint_map: ConstraintFile = serde_json::from_str(json_text).unwrap();
    constraint_map
}
