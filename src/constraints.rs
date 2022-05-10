pub trait Constraint {
    fn new() -> Self where Self: Sized;
    fn check<'c>(&self, field: &'c str) -> Result<&'c str, String>;
}


pub struct Identity {}

impl Constraint for Identity {
    fn new() -> Identity {
        Identity {}
    }

    fn check<'c>(&self, field: &'c str) -> Result<&'c str, String> {
        Ok(field)
    }
}


pub struct NotEmpty {}

impl Constraint for NotEmpty {
    fn new() -> NotEmpty {
        NotEmpty {}
    }

    fn check<'c>(&self, field: &'c str) -> Result<&'c str, String> {
        if field.is_empty() {
            return Err(String::from("Must be non-empty"));
        }
        return Ok(field)
    }
}


pub struct IsInteger {}

impl Constraint for IsInteger {
    fn new() -> IsInteger {
        IsInteger {}
    }

    fn check<'c>(&self, field: &'c str) -> Result<&'c str, String> {
        let res = field.parse::<i64>();
        match res {
            Ok(_) => return Ok(field),
            Err(_) => return Err(String::from("Must be an integer")),
        }
    }
}


pub struct IsFloat {}

impl Constraint for IsFloat {
    fn new() -> IsFloat {
        IsFloat {}
    }

    fn check<'c>(&self, field: &'c str) -> Result<&'c str, String> {
        let res = field.parse::<f64>();
        match res {
            Ok(_) => return Ok(field),
            Err(_) => return Err(String::from("Must be a float")),
        }
    }
}


pub struct IsNumber {}

impl Constraint for IsNumber {
    fn new() -> IsNumber {
        IsNumber {}
    }

    fn check<'c>(&self, field: &'c str) -> Result<&'c str, String> {
        if IsFloat::new().check(field).is_err() {
            return Err(String::from("Must be numeric"));
        }
        return Ok(field)
    }
}


// TESTS
// --------------------------------------------------------
#[cfg(test)]
mod test {
    use super::{
        Constraint,
        Identity,
        IsFloat,
        IsInteger,
        IsNumber,
        NotEmpty,
    };

    #[test]
    fn test_identity() {
        let field = "hi";
        let expect = Ok(field);
        let actual = Identity::new().check(field);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_not_empty_valid() {
        let field = "hi";
        let expect = Ok(field);
        let actual = NotEmpty::new().check(field);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_not_empty_invalid() {
        let field = "";
        let expect = Err(String::from("Must be non-empty"));
        let actual = NotEmpty::new().check(field);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_is_integer_valid() {
        let field = "123";
        let expect = Ok(field);
        let actual = IsInteger::new().check(field);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_is_integer_invalid() {
        let field = "123.321";
        let expect = Err(String::from("Must be an integer"));
        let actual = IsInteger::new().check(field);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_is_float_valid() {
        let field = "123.321";
        let expect = Ok(field);
        let actual = IsFloat::new().check(field);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_is_float_invalid() {
        let field = "hi";
        let expect = Err(String::from("Must be a float"));
        let actual = IsFloat::new().check(field);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_is_number_integer_valid() {
        let field = "123";
        let expect = Ok(field);
        let actual = IsNumber::new().check(field);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_is_number_float_valid() {
        let field = "123.321";
        let expect = Ok(field);
        let actual = IsNumber::new().check(field);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_is_number_invalid() {
        let field = "hi";
        let expect = Err(String::from("Must be numeric"));
        let actual = IsNumber::new().check(field);
        assert_eq!(expect, actual);
    }
}
