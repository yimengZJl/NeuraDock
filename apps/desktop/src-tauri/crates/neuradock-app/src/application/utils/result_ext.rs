use neuradock_domain::shared::DomainError;

/// Extension trait for Result types to simplify error handling
pub trait ResultExt<T, E> {
    /// Convert error to DomainError::Infrastructure
    /// Usage: `result.to_infra_err()?`
    fn to_infra_err(self) -> Result<T, DomainError>;
}

impl<T, E: std::fmt::Display> ResultExt<T, E> for Result<T, E> {
    fn to_infra_err(self) -> Result<T, DomainError> {
        self.map_err(|e| DomainError::Infrastructure(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_infra_err() {
        let result: Result<i32, &str> = Err("infra error");
        let converted = result.to_infra_err();
        match converted {
            Err(DomainError::Infrastructure(msg)) => assert_eq!(msg, "infra error"),
            _ => panic!("Expected Infrastructure error"),
        }
    }
}
