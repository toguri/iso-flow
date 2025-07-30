/// ユーティリティ関数モジュール
pub mod string_utils {
    use regex::Regex;

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

    /// HTMLタグを除去してプレーンテキストに変換
    pub fn strip_html_tags(html: &str) -> String {
        // HTMLタグを除去する正規表現
        let tag_regex = Regex::new(r"<[^>]+>").unwrap();
        let mut text = tag_regex.replace_all(html, " ").to_string();

        // HTMLエンティティをデコード
        text = text
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&#39;", "'")
            .replace("&nbsp;", " ");

        // 連続する空白を1つに
        let whitespace_regex = Regex::new(r"\s+").unwrap();
        text = whitespace_regex.replace_all(&text, " ").to_string();

        // 前後の空白を除去
        text.trim().to_string()
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

    #[test]
    fn test_strip_html_tags() {
        // 基本的なHTMLタグの除去
        assert_eq!(strip_html_tags("<p>Hello World</p>"), "Hello World");
        assert_eq!(
            strip_html_tags("<p>Hello <strong>World</strong></p>"),
            "Hello World"
        );

        // 複数のタグと属性
        assert_eq!(
            strip_html_tags("<div class=\"test\"><p>Hello</p><br/><span>World</span></div>"),
            "Hello World"
        );

        // HTMLエンティティ
        assert_eq!(
            strip_html_tags("&lt;p&gt;Hello &amp; World&lt;/p&gt;"),
            "<p>Hello & World</p>"
        );

        // 実際のRSSフィードからのデータ例
        assert_eq!(
            strip_html_tags("<p>The Miami Heat did not want to include draft capital.</p>\r<p>More content here.</p>"),
            "The Miami Heat did not want to include draft capital. More content here."
        );

        // リンクを含むケース
        assert_eq!(
            strip_html_tags(
                "<p>Check out <a href=\"https://example.com\">this link</a> for more info.</p>"
            ),
            "Check out this link for more info."
        );

        // 空のタグ
        assert_eq!(strip_html_tags("<p></p><div></div>"), "");

        // タグがない場合
        assert_eq!(
            strip_html_tags("Plain text without tags"),
            "Plain text without tags"
        );
    }
}
