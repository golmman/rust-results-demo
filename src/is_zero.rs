use crate::error::{Error, ErrorKind, Result};

fn is_non_negative(i: i32) -> Result<bool> {
    if i < 0 {
        return Err(Error::new(ErrorKind::Negative, "negative!"));
    }
    Ok(true)
}

fn is_non_positive(i: i32) -> Result<bool> {
    if i > 0 {
        return Err(Error::new(ErrorKind::Positive, "positive!"));
    }
    Ok(true)
}

pub fn is_zero(i: i32) -> Result<bool> {
    is_non_negative(i)?;
    is_non_positive(i)?;

    Ok(true)
}

pub fn is_zero2(
    i: i32,
) -> std::result::Result<bool, Box<dyn std::error::Error>> {
    is_non_negative(i)?;
    is_non_positive(i)?;

    Ok(true)
}

#[cfg(test)]
mod tests {
    mod is_zero_tests {
        use super::super::*;

        #[test]
        fn ok() {
            let zero = is_zero(0);
            assert_eq!(true, zero.is_ok());
            assert_eq!(true, zero.unwrap());
        }

        #[test]
        fn negative() {
            let neg = is_zero(-5);
            assert_eq!(true, neg.is_err());
            assert_eq!(ErrorKind::Negative, neg.unwrap_err().kind());
        }

        #[test]
        fn positive() {
            let pos = is_zero(5);
            assert_eq!(true, pos.is_err());
            assert_eq!(ErrorKind::Positive, pos.unwrap_err().kind());
        }
    }
}
