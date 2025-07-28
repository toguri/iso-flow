/// ユーティリティ関数モジュール
pub mod string_utils {
    /// 文字列を大文字に変換
    pub fn to_uppercase(s: &str) -> String {
        s.to_uppercase()
    }

    /// 文字列を小文字に変換
    pub fn to_lowercase(s: &str) -> String {
        s.to_lowercase()
    }

    /// 文字列の前後の空白を除去
    pub fn trim(s: &str) -> String {
        s.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::string_utils::*;

    #[test]
    fn test_to_uppercase() {
        assert_eq!(to_uppercase("hello"), "HELLO");
        assert_eq!(to_uppercase("World"), "WORLD");
    }

    #[test]
    fn test_to_lowercase() {
        assert_eq!(to_lowercase("HELLO"), "hello");
        assert_eq!(to_lowercase("World"), "world");
    }

    #[test]
    fn test_trim() {
        assert_eq!(trim("  hello  "), "hello");
        assert_eq!(trim("\tworld\n"), "world");
        assert_eq!(trim("no_space"), "no_space");
        assert_eq!(trim(""), "");
        assert_eq!(trim("   "), "");
    }
}
