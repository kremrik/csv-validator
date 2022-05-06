use crate::constraints as cst;
use csv::{
    StringRecord,
};
use std::collections::HashMap;


type Constraint = fn(&str)-> Result<&str, String>;
type Constraints = HashMap<String, Vec<Constraint>>;


struct ValidationError {
    row: usize,
    name: String,
    value: String,
    message: Vec<String>,
}
