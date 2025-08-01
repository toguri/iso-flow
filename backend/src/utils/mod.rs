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

    #[test]
    fn test_strip_html_tags_edge_cases() {
        // ネストしたタグ
        assert_eq!(
            strip_html_tags("<div><p><span>Nested</span> <em>tags</em></p></div>"),
            "Nested tags"
        );

        // 自己終了タグ
        assert_eq!(
            strip_html_tags("Text<br/>with<hr/>breaks"),
            "Text with breaks"
        );

        // コメント
        assert_eq!(
            strip_html_tags("<!-- comment -->Text<!-- another comment -->"),
            "Text"
        );

        // スクリプトタグ
        assert_eq!(
            strip_html_tags("<script>alert('test');</script>Safe text"),
            "alert('test'); Safe text"
        );

        // スタイルタグ
        assert_eq!(
            strip_html_tags("<style>body { color: red; }</style>Content"),
            "body { color: red; } Content"
        );

        // 壊れたHTML
        assert_eq!(
            strip_html_tags("<p>Unclosed paragraph"),
            "Unclosed paragraph"
        );

        // 特殊なHTMLエンティティ
        assert_eq!(
            strip_html_tags("&#39;Single&#39; &quot;Double&quot; &nbsp;Space"),
            "'Single' \"Double\" Space"
        );

        // 連続する空白の処理
        assert_eq!(
            strip_html_tags("<p>Multiple   spaces</p>\n\n<p>and   lines</p>"),
            "Multiple spaces and lines"
        );
    }

    #[test]
    fn test_strip_html_tags_with_attributes() {
        // 様々な属性を持つタグ
        assert_eq!(
            strip_html_tags("<div id='test' class=\"container\" data-value='123'>Content</div>"),
            "Content"
        );

        // インラインスタイル
        assert_eq!(
            strip_html_tags("<p style='color: red; font-size: 14px;'>Styled text</p>"),
            "Styled text"
        );

        // イベントハンドラ
        assert_eq!(
            strip_html_tags("<button onclick='doSomething()'>Click me</button>"),
            "Click me"
        );
    }

    #[test]
    fn test_to_uppercase_unicode() {
        // Unicode文字のテスト
        assert_eq!(to_uppercase("こんにちは"), "こんにちは"); // 日本語は変化しない
        assert_eq!(to_uppercase("café"), "CAFÉ");
        assert_eq!(to_uppercase("αβγ"), "ΑΒΓ");
    }

    #[test]
    fn test_to_lowercase_unicode() {
        // Unicode文字のテスト
        assert_eq!(to_lowercase("こんにちは"), "こんにちは"); // 日本語は変化しない
        assert_eq!(to_lowercase("CAFÉ"), "café");
        assert_eq!(to_lowercase("ΑΒΓ"), "αβγ");
    }

    #[test]
    fn test_trim_various_whitespace() {
        // 様々な空白文字
        assert_eq!(trim("\r\ntext\r\n"), "text");
        assert_eq!(trim("\u{00A0}text\u{00A0}"), "text"); // Non-breaking space is trimmed
        assert_eq!(trim("\t\t\ttext\t\t\t"), "text");
        assert_eq!(trim(" \n \r \t text \t \r \n "), "text");
    }
}
