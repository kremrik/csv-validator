use crate::constraints as cst;
use csv::{
    StringRecord,
};
use std::collections::HashMap;


type Constraint = for<'r> fn(&'r str) -> Result<&'r str, String>;


struct ValidationError {
    row: usize,
    name: String,
    value: String,
    message: Vec<String>,
}
