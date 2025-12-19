use neuradock_domain::shared::DomainError;

/// Extension trait for Result types to simplify error handling
pub trait ResultExt<T, E> {
    /// Convert error to String for presentation layer
    /// Usage: `result.to_string_err()?`
    fn to_string_err(self) -> Result<T, String>;

    /// Convert error to DomainError::Repository
    /// Usage: `result.to_repo_err()?`
    fn to_repo_err(self) -> Result<T, DomainError>;

    /// Convert error to DomainError::Infrastructure
    /// Usage: `result.to_infra_err()?`
    fn to_infra_err(self) -> Result<T, DomainError>;

    /// Convert error to DomainError::Validation
    /// Usage: `result.to_validation_err()?`
    fn to_validation_err(self) -> Result<T, DomainError>;
}

impl<T, E: std::fmt::Display> ResultExt<T, E> for Result<T, E> {
    fn to_string_err(self) -> Result<T, String> {
        self.map_err(|e| e.to_string())
    }

    fn to_repo_err(self) -> Result<T, DomainError> {
        self.map_err(|e| DomainError::Repository(e.to_string()))
    }

    fn to_infra_err(self) -> Result<T, DomainError> {
        self.map_err(|e| DomainError::Infrastructure(e.to_string()))
    }

    fn to_validation_err(self) -> Result<T, DomainError> {
        self.map_err(|e| DomainError::Validation(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string_err() {
        let result: Result<i32, &str> = Err("test error");
        let converted = result.to_string_err();
        assert_eq!(converted.unwrap_err(), "test error");
    }

    #[test]
    fn test_to_repo_err() {
        let result: Result<i32, &str> = Err("repo error");
        let converted = result.to_repo_err();
        match converted {
            Err(DomainError::Repository(msg)) => assert_eq!(msg, "repo error"),
            _ => panic!("Expected Repository error"),
        }
    }

    #[test]
    fn test_to_infra_err() {
        let result: Result<i32, &str> = Err("infra error");
        let converted = result.to_infra_err();
        match converted {
            Err(DomainError::Infrastructure(msg)) => assert_eq!(msg, "infra error"),
            _ => panic!("Expected Infrastructure error"),
        }
    }

    #[test]
    fn test_to_validation_err() {
        let result: Result<i32, &str> = Err("validation error");
        let converted = result.to_validation_err();
        match converted {
            Err(DomainError::Validation(msg)) => assert_eq!(msg, "validation error"),
            _ => panic!("Expected Validation error"),
        }
    }
}
