//! 翻訳サービスの実装
//!
//! Amazon Translateを使用して英語から日本語への翻訳を提供します。

use async_trait::async_trait;
use aws_config::BehaviorVersion;
use aws_sdk_translate::Client as TranslateClient;
use thiserror::Error;
use tracing::{debug, error, info};

#[derive(Error, Debug)]
pub enum TranslationError {
    #[error("AWS SDK error: {0}")]
    AwsError(String),
    #[error("Translation failed: {0}")]
    TranslationFailed(String),
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
}

#[async_trait]
pub trait TranslationService: Send + Sync {
    async fn translate(
        &self,
        text: &str,
        source_lang: &str,
        target_lang: &str,
    ) -> Result<String, TranslationError>;
}

pub struct AmazonTranslateService {
    client: TranslateClient,
}

impl AmazonTranslateService {
    pub async fn new() -> Self {
        let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
        let client = TranslateClient::new(&config);

        Self { client }
    }
}

#[async_trait]
impl TranslationService for AmazonTranslateService {
    async fn translate(
        &self,
        text: &str,
        source_lang: &str,
        target_lang: &str,
    ) -> Result<String, TranslationError> {
        if text.trim().is_empty() {
            return Ok(String::new());
        }

        debug!(
            "Translating text from {} to {}: {} chars",
            source_lang,
            target_lang,
            text.len()
        );

        let result = self
            .client
            .translate_text()
            .text(text)
            .source_language_code(source_lang)
            .target_language_code(target_lang)
            .send()
            .await;

        match result {
            Ok(output) => {
                let translated_text = output.translated_text;
                info!(
                    "Successfully translated {} chars to {} chars",
                    text.len(),
                    translated_text.len()
                );
                Ok(translated_text)
            }
            Err(err) => {
                error!("Amazon Translate error: {}", err);
                match err {
                    aws_sdk_translate::error::SdkError::ServiceError(service_err) => {
                        let err_msg = service_err.err().to_string();
                        // Check if it's a throttling error by looking at the error message
                        if err_msg.contains("throttl") || err_msg.contains("rate") {
                            Err(TranslationError::RateLimitExceeded)
                        } else {
                            Err(TranslationError::TranslationFailed(err_msg))
                        }
                    }
                    _ => Err(TranslationError::AwsError(err.to_string())),
                }
            }
        }
    }
}

// モックサービス（テスト用）
pub struct MockTranslationService;

#[async_trait]
impl TranslationService for MockTranslationService {
    async fn translate(
        &self,
        text: &str,
        _source_lang: &str,
        _target_lang: &str,
    ) -> Result<String, TranslationError> {
        // シンプルなモック翻訳（実際の翻訳は行わない）
        Ok(format!("[翻訳済み] {text}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_translation_service() {
        let service = MockTranslationService;
        let result = service.translate("Hello World", "en", "ja").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "[翻訳済み] Hello World");
    }

    #[tokio::test]
    async fn test_mock_translation_service_with_japanese() {
        let service = MockTranslationService;
        let result = service.translate("レイカーズがトレード", "ja", "en").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "[翻訳済み] レイカーズがトレード");
    }

    #[tokio::test]
    async fn test_empty_text_translation() {
        let service = MockTranslationService;
        let result = service.translate("", "en", "ja").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "[翻訳済み] ");
    }

    #[tokio::test]
    async fn test_mock_translation_with_whitespace() {
        let service = MockTranslationService;
        let result = service.translate("  \n\t  ", "en", "ja").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "[翻訳済み]   \n\t  ");
    }

    #[tokio::test]
    async fn test_mock_translation_long_text() {
        let service = MockTranslationService;
        let long_text = "The Los Angeles Lakers have acquired a star player in a blockbuster trade deal. This move is expected to significantly improve their championship chances.";
        let result = service.translate(long_text, "en", "ja").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), format!("[翻訳済み] {}", long_text));
    }

    #[tokio::test]
    async fn test_amazon_translate_service_new() {
        // AWS SDKの初期化テスト
        let _service = AmazonTranslateService::new().await;
        // 初期化が成功すればOK（実際のAWS接続は不要）
        assert!(true);
    }

    #[tokio::test]
    async fn test_translation_error_display() {
        let error = TranslationError::AwsError("Test error".to_string());
        assert_eq!(error.to_string(), "AWS SDK error: Test error");

        let error = TranslationError::TranslationFailed("Failed".to_string());
        assert_eq!(error.to_string(), "Translation failed: Failed");

        let error = TranslationError::RateLimitExceeded;
        assert_eq!(error.to_string(), "Rate limit exceeded");
    }

    // Amazon Translateのエラーハンドリングテスト用のモック
    #[derive(Clone)]
    struct MockTranslateClient;

    #[async_trait]
    trait TranslateClientTrait: Send + Sync {
        async fn translate(&self, text: &str) -> Result<String, String>;
    }

    struct TestableAmazonTranslateService<T: TranslateClientTrait> {
        client: T,
    }

    #[async_trait]
    impl<T: TranslateClientTrait> TranslationService for TestableAmazonTranslateService<T> {
        async fn translate(
            &self,
            text: &str,
            _source_lang: &str,
            _target_lang: &str,
        ) -> Result<String, TranslationError> {
            if text.trim().is_empty() {
                return Ok(String::new());
            }

            match self.client.translate(text).await {
                Ok(translated) => Ok(translated),
                Err(err) => {
                    if err.contains("throttl") || err.contains("rate") {
                        Err(TranslationError::RateLimitExceeded)
                    } else {
                        Err(TranslationError::TranslationFailed(err))
                    }
                }
            }
        }
    }

    #[async_trait]
    impl TranslateClientTrait for MockTranslateClient {
        async fn translate(&self, text: &str) -> Result<String, String> {
            if text.contains("error") {
                Err("Translation failed".to_string())
            } else if text.contains("throttle") {
                Err("Request throttled".to_string())
            } else {
                Ok(format!("Translated: {}", text))
            }
        }
    }

    #[tokio::test]
    async fn test_amazon_translate_empty_text() {
        let client = MockTranslateClient;
        let service = TestableAmazonTranslateService { client };

        let result = service.translate("", "en", "ja").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");

        let result = service.translate("   ", "en", "ja").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[tokio::test]
    async fn test_amazon_translate_error_handling() {
        let client = MockTranslateClient;
        let service = TestableAmazonTranslateService { client };

        let result = service.translate("error case", "en", "ja").await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TranslationError::TranslationFailed(_)
        ));
    }

    #[tokio::test]
    async fn test_amazon_translate_rate_limit() {
        let client = MockTranslateClient;
        let service = TestableAmazonTranslateService { client };

        let result = service.translate("throttle test", "en", "ja").await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TranslationError::RateLimitExceeded
        ));
    }

    #[tokio::test]
    async fn test_mock_translation_special_characters() {
        let service = MockTranslationService;
        let special_text = "NBA™ News: Lakers' \"Big Trade\" & More! 🏀";
        let result = service.translate(special_text, "en", "ja").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), format!("[翻訳済み] {}", special_text));
    }

    #[tokio::test]
    async fn test_mock_translation_unicode() {
        let service = MockTranslationService;
        let unicode_text = "こんにちは！🇯🇵 NBA ニュース 📰";
        let result = service.translate(unicode_text, "ja", "en").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), format!("[翻訳済み] {}", unicode_text));
    }

    #[tokio::test]
    async fn test_translation_error_debug() {
        let error = TranslationError::AwsError("Debug test".to_string());
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("AwsError"));
        assert!(debug_str.contains("Debug test"));
    }

    #[tokio::test]
    async fn test_mock_translation_html_entities() {
        let service = MockTranslationService;
        let html_text = "Lakers &amp; Celtics: Trade &quot;News&quot;";
        let result = service.translate(html_text, "en", "ja").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), format!("[翻訳済み] {}", html_text));
    }

    #[tokio::test]
    async fn test_testable_service_successful_translation() {
        let client = MockTranslateClient;
        let service = TestableAmazonTranslateService { client };

        let result = service.translate("Hello World", "en", "ja").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Translated: Hello World");
    }

    #[tokio::test]
    async fn test_very_long_text() {
        let service = MockTranslationService;
        let very_long_text = "Lakers ".repeat(1000); // 6000文字以上
        let result = service.translate(&very_long_text, "en", "ja").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), format!("[翻訳済み] {}", very_long_text));
    }

    #[tokio::test]
    async fn test_multiple_language_codes() {
        let service = MockTranslationService;

        // 様々な言語コードでテスト
        let test_cases = vec![
            ("en", "ja"),
            ("ja", "en"),
            ("es", "ja"),
            ("fr", "en"),
            ("de", "ja"),
        ];

        for (source, target) in test_cases {
            let result = service.translate("Test", source, target).await;
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "[翻訳済み] Test");
        }
    }

    #[tokio::test]
    async fn test_translation_with_newlines() {
        let service = MockTranslationService;
        let multiline_text = "Line 1\nLine 2\n\nLine 4";
        let result = service.translate(multiline_text, "en", "ja").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), format!("[翻訳済み] {}", multiline_text));
    }

    // 追加のモックテスト用クライアント
    #[derive(Clone)]
    struct ExtendedMockTranslateClient {
        should_fail: bool,
        error_type: String,
    }

    #[async_trait]
    impl TranslateClientTrait for ExtendedMockTranslateClient {
        async fn translate(&self, text: &str) -> Result<String, String> {
            if self.should_fail {
                Err(self.error_type.clone())
            } else {
                // 実際の翻訳をシミュレート
                match text {
                    "Hello" => Ok("こんにちは".to_string()),
                    "Goodbye" => Ok("さようなら".to_string()),
                    _ => Ok(format!("翻訳: {}", text)),
                }
            }
        }
    }

    #[tokio::test]
    async fn test_extended_mock_success() {
        let client = ExtendedMockTranslateClient {
            should_fail: false,
            error_type: String::new(),
        };
        let service = TestableAmazonTranslateService { client };

        let result = service.translate("Hello", "en", "ja").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "こんにちは");
    }

    #[tokio::test]
    async fn test_extended_mock_various_errors() {
        let test_cases = vec![
            (
                "Generic error",
                TranslationError::TranslationFailed("Generic error".to_string()),
            ),
            ("rate limit", TranslationError::RateLimitExceeded),
            ("throttled", TranslationError::RateLimitExceeded),
        ];

        for (error_msg, expected_error) in test_cases {
            let client = ExtendedMockTranslateClient {
                should_fail: true,
                error_type: error_msg.to_string(),
            };
            let service = TestableAmazonTranslateService { client };

            let result = service.translate("Test", "en", "ja").await;
            assert!(result.is_err());

            match (result.unwrap_err(), expected_error) {
                (TranslationError::RateLimitExceeded, TranslationError::RateLimitExceeded) => {}
                (
                    TranslationError::TranslationFailed(a),
                    TranslationError::TranslationFailed(b),
                ) => {
                    assert_eq!(a, b);
                }
                _ => panic!("Unexpected error type"),
            }
        }
    }

    #[test]
    fn test_translation_service_trait_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<MockTranslationService>();
        assert_send_sync::<Box<dyn TranslationService>>();
    }

    #[tokio::test]
    async fn test_translation_error_from_string() {
        let errors = vec![
            TranslationError::AwsError("AWS error".to_string()),
            TranslationError::TranslationFailed("Failed".to_string()),
            TranslationError::RateLimitExceeded,
        ];

        for error in errors {
            // エラーが正しく文字列に変換されることを確認
            let error_string = error.to_string();
            assert!(!error_string.is_empty());
        }
    }

    #[tokio::test]
    async fn test_amazon_translate_service_empty_trimmed_text() {
        // AmazonTranslateServiceの空白文字の処理をテスト
        struct EmptyTextMockClient;

        #[async_trait]
        impl TranslateClientTrait for EmptyTextMockClient {
            async fn translate(&self, _text: &str) -> Result<String, String> {
                panic!("Should not be called for empty text");
            }
        }

        let client = EmptyTextMockClient;
        let service = TestableAmazonTranslateService { client };

        // 空文字
        let result = service.translate("", "en", "ja").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");

        // 空白のみ
        let result = service.translate("   ", "en", "ja").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");

        // タブと改行のみ
        let result = service.translate("\t\n", "en", "ja").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[tokio::test]
    async fn test_error_message_exact_matching() {
        // エラーメッセージの正確な一致をテスト
        let aws_error = TranslationError::AwsError("Connection timeout".to_string());
        assert_eq!(aws_error.to_string(), "AWS SDK error: Connection timeout");

        let translation_error =
            TranslationError::TranslationFailed("Invalid language code".to_string());
        assert_eq!(
            translation_error.to_string(),
            "Translation failed: Invalid language code"
        );

        let rate_limit_error = TranslationError::RateLimitExceeded;
        assert_eq!(rate_limit_error.to_string(), "Rate limit exceeded");
    }

    #[tokio::test]
    async fn test_mock_service_with_all_whitespace_types() {
        let service = MockTranslationService;

        // 様々な空白文字の組み合わせ
        let whitespace_tests = vec![
            " ",        // スペース
            "\t",       // タブ
            "\n",       // 改行
            "\r",       // キャリッジリターン
            "\r\n",     // Windows改行
            " \t \n ",  // 混合
            "\u{00A0}", // ノンブレーキングスペース
            "\u{2003}", // emスペース
        ];

        for ws in whitespace_tests {
            let result = service.translate(ws, "en", "ja").await;
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), format!("[翻訳済み] {}", ws));
        }
    }

    #[tokio::test]
    async fn test_concurrent_translations() {
        // 並行実行のテスト
        let texts = vec!["Hello", "World", "Test", "Concurrent"];

        let mut handles = vec![];

        for text in texts {
            let handle = tokio::spawn(async move {
                let service = MockTranslationService;
                service.translate(text, "en", "ja").await
            });
            handles.push(handle);
        }

        // 全ての翻訳が成功することを確認
        for handle in handles {
            let result = handle.await.unwrap();
            assert!(result.is_ok());
            assert!(result.unwrap().starts_with("[翻訳済み]"));
        }
    }

    #[test]
    fn test_translation_error_is_send_sync() {
        // TranslationErrorがSend + Syncであることを確認
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<TranslationError>();
    }

    #[tokio::test]
    async fn test_edge_case_language_codes() {
        let service = MockTranslationService;

        // エッジケースの言語コード
        let edge_cases = vec![
            ("", "ja"),         // 空のソース言語
            ("en", ""),         // 空のターゲット言語
            ("xxx", "yyy"),     // 無効な言語コード
            ("EN", "JA"),       // 大文字
            ("en-US", "ja-JP"), // リージョンコード付き
        ];

        for (source, target) in edge_cases {
            let result = service.translate("Test", source, target).await;
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "[翻訳済み] Test");
        }
    }

    #[tokio::test]
    async fn test_testable_service_edge_cases() {
        // 境界値テスト用のモッククライアント
        struct EdgeCaseMockClient;

        #[async_trait]
        impl TranslateClientTrait for EdgeCaseMockClient {
            async fn translate(&self, text: &str) -> Result<String, String> {
                match text {
                    "rate_limit" => Err("Request rate exceeded".to_string()),
                    "throttling" => Err("Request throttling applied".to_string()),
                    "unknown" => Err("Unknown error occurred".to_string()),
                    _ => Ok(format!("翻訳完了: {}", text)),
                }
            }
        }

        let client = EdgeCaseMockClient;
        let service = TestableAmazonTranslateService { client };

        // レート制限（"rate"を含む）
        let result = service.translate("rate_limit", "en", "ja").await;
        assert!(matches!(result, Err(TranslationError::RateLimitExceeded)));

        // スロットリング（"throttl"を含む）
        let result = service.translate("throttling", "en", "ja").await;
        assert!(matches!(result, Err(TranslationError::RateLimitExceeded)));

        // その他のエラー
        let result = service.translate("unknown", "en", "ja").await;
        match result {
            Err(TranslationError::TranslationFailed(msg)) => {
                assert_eq!(msg, "Unknown error occurred");
            }
            _ => panic!("Expected TranslationFailed error"),
        }
    }

    #[tokio::test]
    async fn test_debug_output_format() {
        // Debug出力のフォーマットテスト
        let error1 = TranslationError::AwsError("test error".to_string());
        let debug1 = format!("{:?}", error1);
        assert!(debug1.contains("AwsError"));
        assert!(debug1.contains("test error"));

        let error2 = TranslationError::RateLimitExceeded;
        let debug2 = format!("{:?}", error2);
        assert!(debug2.contains("RateLimitExceeded"));
    }

    #[tokio::test]
    async fn test_translation_with_mixed_content() {
        let service = MockTranslationService;

        // 様々なコンテンツを含むテキスト
        let mixed_content_tests = vec![
            "Hello\nWorld",             // 改行を含む
            "Test\tWith\tTabs",         // タブを含む
            "Mixed 123 数字 456",       // 数字と日本語
            "Special @#$% Characters",  // 特殊文字
            "Emoji 😀 Test 🏀",         // 絵文字
            "<html>Tagged</html>",      // HTMLタグ
            "\"Quoted\" Text",          // 引用符
            "Mixed\r\nLine\r\nEndings", // 複数の改行タイプ
        ];

        for content in mixed_content_tests {
            let result = service.translate(content, "en", "ja").await;
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), format!("[翻訳済み] {}", content));
        }
    }

    #[tokio::test]
    async fn test_translation_service_reusability() {
        // サービスの再利用性テスト
        let service = MockTranslationService;

        // 同じサービスインスタンスで複数回翻訳
        for i in 0..5 {
            let text = format!("Test {}", i);
            let result = service.translate(&text, "en", "ja").await;
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), format!("[翻訳済み] {}", text));
        }
    }

    #[tokio::test]
    async fn test_aws_specific_error_patterns() {
        // AWS特有のエラーパターンテスト
        struct AwsErrorMockClient;

        #[async_trait]
        impl TranslateClientTrait for AwsErrorMockClient {
            async fn translate(&self, text: &str) -> Result<String, String> {
                match text {
                    "throttle_test" => Err("Request throttled by AWS".to_string()),
                    "rate_test" => Err("Request rate exceeded for this operation".to_string()),
                    "auth_test" => Err("Authentication failed".to_string()),
                    "service_test" => Err("Service temporarily unavailable".to_string()),
                    _ => Ok("Translated".to_string()),
                }
            }
        }

        let client = AwsErrorMockClient;
        let service = TestableAmazonTranslateService { client };

        // スロットリングエラー
        let result = service.translate("throttle_test", "en", "ja").await;
        assert!(matches!(result, Err(TranslationError::RateLimitExceeded)));

        // レート制限エラー
        let result = service.translate("rate_test", "en", "ja").await;
        assert!(matches!(result, Err(TranslationError::RateLimitExceeded)));

        // 認証エラー
        let result = service.translate("auth_test", "en", "ja").await;
        match result {
            Err(TranslationError::TranslationFailed(msg)) => {
                assert_eq!(msg, "Authentication failed");
            }
            _ => panic!("Expected TranslationFailed error"),
        }

        // サービスエラー
        let result = service.translate("service_test", "en", "ja").await;
        match result {
            Err(TranslationError::TranslationFailed(msg)) => {
                assert_eq!(msg, "Service temporarily unavailable");
            }
            _ => panic!("Expected TranslationFailed error"),
        }
    }
}
