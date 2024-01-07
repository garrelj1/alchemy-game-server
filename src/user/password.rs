use std::fmt;

#[derive(Debug)]
pub enum PasswordValidationError {
    TooShort,
    TooLong,
    NoUppercase,
    NoLowercase,
    NoDigit,
    NoSpecialCharacter,
}


impl fmt::Display for PasswordValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Customize the error message for each variant
        match self {
            PasswordValidationError::TooShort => write!(f, "Password is too short"),
            PasswordValidationError::TooLong => write!(f, "Password is too long"),
            PasswordValidationError::NoUppercase => write!(f, "Password must contain at least one uppercase letter"),
            PasswordValidationError::NoLowercase => write!(f, "Password must contain at least one lowercase letter"),
            PasswordValidationError::NoDigit => write!(f, "Password must contain at least one digit"),
            PasswordValidationError::NoSpecialCharacter => write!(f, "Password must contain at least one special character"),
            // Add more error messages as needed
        }
    }
}
impl std::error::Error for PasswordValidationError {}