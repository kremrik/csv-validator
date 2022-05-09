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


fn sort_constraints(
    headers: &StringRecord,
    constraints: &HashMap<String, Constraint>,
) -> Vec<Constraint> {
    let mut output = Vec::new();
    let default = cst::identity as fn(&str) -> Result<&str, String>;

    for col_name in headers.iter() {
        let constraint = constraints.get(col_name).unwrap_or(&default);
        output.push(*constraint);
    }

    output
}


// TESTS
// --------------------------------------------------------
#[cfg(test)]
mod test {
    use super::{
        HashMap,
        StringRecord,
        cst,
        sort_constraints,
    };

    #[test]
    fn test_sort_constraints() {
        let headers = StringRecord::from(vec!["one", "two"]);
        let constraints = HashMap::from([
            (String::from("two"), cst::is_float as fn(&str) -> Result<&str, String>),
            (String::from("one"), cst::not_empty as fn(&str) -> Result<&str, String>),
        ]);
        let expect = vec![
            cst::not_empty as fn(&str) -> Result<&str, String>,
            cst::is_float as fn(&str) -> Result<&str, String>,
        ];
        let actual = sort_constraints(&headers, &constraints);
        assert_eq!(expect, actual);
    }
}
