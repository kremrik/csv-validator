#[derive(Clone, Copy, Debug)]
pub enum Constraint {
    Identity,
    NotEmpty,
    IsInteger,
    IsFloat,
    IsNumber,
}

pub fn check<'c>(value: &'c str, constraint: &'c Constraint) -> Result<&'c str, String> {
    match constraint {
        Constraint::Identity => identity(value),
        Constraint::NotEmpty => not_empty(value),
        Constraint::IsInteger => is_integer(value),
        Constraint::IsFloat => is_float(value),
        Constraint::IsNumber => is_number(value),
    }
}

fn identity(field: &str) -> Result<&str, String> {
    Ok(field)
}

fn not_empty(field: &str) -> Result<&str, String> {
    if field.is_empty() {
        return Err(String::from("Must be non-empty"));
    }
    return Ok(field);
}

fn is_integer(field: &str) -> Result<&str, String> {
    let res = field.parse::<i64>();
    match res {
        Ok(_) => return Ok(field),
        Err(_) => return Err(String::from("Must be an integer")),
    }
}

fn is_float(field: &str) -> Result<&str, String> {
    let res = field.parse::<f64>();
    match res {
        Ok(_) => return Ok(field),
        Err(_) => return Err(String::from("Must be a float")),
    }
}

fn is_number(field: &str) -> Result<&str, String> {
    if is_float(field).is_err() {
        return Err(String::from("Must be numeric"));
    }
    return Ok(field);
}

// TESTS
// --------------------------------------------------------
#[cfg(test)]
mod test {
    use super::{check, Constraint};

    #[test]
    fn test_identity() {
        let field = "hi";
        let expect = Ok(field);
        let actual = check(field, &Constraint::Identity);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_not_empty_valid() {
        let field = "hi";
        let expect = Ok(field);
        let actual = check(field, &Constraint::NotEmpty);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_not_empty_invalid() {
        let field = "";
        let expect = Err(String::from("Must be non-empty"));
        let actual = check(field, &Constraint::NotEmpty);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_is_integer_valid() {
        let field = "123";
        let expect = Ok(field);
        let actual = check(field, &Constraint::IsInteger);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_is_integer_invalid() {
        let field = "123.321";
        let expect = Err(String::from("Must be an integer"));
        let actual = check(field, &Constraint::IsInteger);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_is_float_valid() {
        let field = "123.321";
        let expect = Ok(field);
        let actual = check(field, &Constraint::IsFloat);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_is_float_invalid() {
        let field = "hi";
        let expect = Err(String::from("Must be a float"));
        let actual = check(field, &Constraint::IsFloat);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_is_number_integer_valid() {
        let field = "123";
        let expect = Ok(field);
        let actual = check(field, &Constraint::IsNumber);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_is_number_float_valid() {
        let field = "123.321";
        let expect = Ok(field);
        let actual = check(field, &Constraint::IsNumber);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_is_number_invalid() {
        let field = "hi";
        let expect = Err(String::from("Must be numeric"));
        let actual = check(field, &Constraint::IsNumber);
        assert_eq!(expect, actual);
    }
}
