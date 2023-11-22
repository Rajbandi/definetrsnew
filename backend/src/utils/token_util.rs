// token_util.rs

pub struct TokenUtil;

impl TokenUtil {
    pub fn extract_hex_string(input: &str) -> Option<String> {
        if input.starts_with("Bytes(\"") && input.ends_with("\")") {
            let start = "Bytes(\"".len();
            let end = input.len() - "\")".len();
            Some(input[start..end].to_string())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_hex_string() {
        assert_eq!(
            TokenUtil::extract_hex_string("Bytes(\"48656c6c6f\")"),
            Some("48656c6c6f".to_string())
        );
        assert_eq!(TokenUtil::extract_hex_string("Bytes(\"invalid)"), None);
    }
}
