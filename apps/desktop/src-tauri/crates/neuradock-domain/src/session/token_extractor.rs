use std::cmp::Reverse;
use std::collections::HashMap;

/// Service to extract the most likely session token from a set of cookies
pub struct SessionTokenExtractor;

impl SessionTokenExtractor {
    /// Common session cookie names in priority order
    const SESSION_KEYS: &'static [&'static str] = &[
        "session",
        "session_id",
        "sessionid",
        "token",
        "access_token",
        "auth",
        "authorization",
        "jwt",
        "connect.sid",
        "PHPSESSID",
        "JSESSIONID",
    ];

    /// Extract session token from cookies
    ///
    /// Strategy:
    /// 1. Look for known session keys (case-insensitive)
    /// 2. If not found, look for the longest value (heuristic: tokens are usually long)
    /// 3. Fallback to the first value found
    /// 4. Return "session" literal if map is empty (legacy fallback)
    pub fn extract(cookies: &HashMap<String, String>) -> String {
        if cookies.is_empty() {
            return "session".to_string();
        }

        // 1. Priority keys
        for &key in Self::SESSION_KEYS {
            // Exact match
            if let Some(val) = cookies.get(key) {
                if !val.is_empty() {
                    return val.clone();
                }
            }
            // Case-insensitive match
            if let Some((_, val)) = cookies
                .iter()
                .find(|(k, _)| k.to_lowercase() == key.to_lowercase())
            {
                if !val.is_empty() {
                    return val.clone();
                }
            }
        }

        // 2. Longest value heuristic
        // Sort by length descending
        let mut values: Vec<&String> = cookies.values().filter(|v| !v.is_empty()).collect();
        if !values.is_empty() {
            values.sort_by_key(|val| Reverse(val.len()));
            return values[0].clone();
        }

        // 3. Fallback
        cookies
            .values()
            .next()
            .cloned()
            .unwrap_or_else(|| "session".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_priority_key() {
        let mut cookies = HashMap::new();
        cookies.insert("other".to_string(), "123".to_string());
        cookies.insert("sessionid".to_string(), "secret_token".to_string());

        assert_eq!(SessionTokenExtractor::extract(&cookies), "secret_token");
    }

    #[test]
    fn test_extract_longest_heuristic() {
        let mut cookies = HashMap::new();
        cookies.insert("theme".to_string(), "dark".to_string());
        cookies.insert(
            "unknown_cookie".to_string(),
            "very_long_random_string_that_looks_like_a_token".to_string(),
        );

        assert_eq!(
            SessionTokenExtractor::extract(&cookies),
            "very_long_random_string_that_looks_like_a_token"
        );
    }

    #[test]
    fn test_empty_cookies() {
        let cookies = HashMap::new();
        assert_eq!(SessionTokenExtractor::extract(&cookies), "session");
    }
}
