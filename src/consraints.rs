pub fn not_empty(field: &str) -> Result<(), String> {
    if field.is_empty() {
        return Err(String::from("Must be non-empty"));
    }
    return Ok(())
}

pub fn is_integer(field: &str) -> Result<(), String> {
    let res = field.parse::<i64>();
    match res {
        Ok(_) => return Ok(()),
        Err(_) => {
            let msg = format!("Must be an integer, received [{field}]");
            return Err(msg);
        },
    }
}

pub fn is_float(field: &str) -> Result<(), String> {
    let res = field.parse::<f64>();
    match res {
        Ok(_) => return Ok(()),
        Err(_) => {
            let msg = format!("Must be a float, received [{field}]");
            return Err(msg);
        },
    }
}

pub fn is_number(field: &str) -> Result<(), String> {
    if is_float(field).is_err() {
        return Err(format!("Must be numeric, received [{field}]"));
    }
    return Ok(())
}

// TESTS
// --------------------------------------------------------
#[cfg(test)]
mod test {
    use super::{
        is_float,
        is_integer,
        is_number,
        not_empty
    };

    #[test]
    fn test_not_empty_valid() {
        let field = "hi";
        let expect = Ok(());
        let actual = not_empty(field);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_not_empty_invalid() {
        let field = "";
        let expect = Err(String::from("Must be non-empty"));
        let actual = not_empty(field);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_is_integer_valid() {
        let field = "123";
        let expect = Ok(());
        let actual = is_integer(field);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_is_integer_invalid() {
        let field = "123.321";
        let expect = Err(String::from("Must be an integer, received [123.321]"));
        let actual = is_integer(field);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_is_float_valid() {
        let field = "123.321";
        let expect = Ok(());
        let actual = is_float(field);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_is_float_invalid() {
        let field = "hi";
        let expect = Err(String::from("Must be a float, received [hi]"));
        let actual = is_float(field);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_is_number_integer_valid() {
        let field = "123";
        let expect = Ok(());
        let actual = is_number(field);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_is_number_float_valid() {
        let field = "123.321";
        let expect = Ok(());
        let actual = is_number(field);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_is_number_invalid() {
        let field = "hi";
        let expect = Err(String::from("Must be numeric, received [hi]"));
        let actual = is_number(field);
        assert_eq!(expect, actual);
    }
}
