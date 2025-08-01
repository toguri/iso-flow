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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_amazon_translate_real_connection() {
        // 実際のAWS Translateサービスを使用
        let service = AmazonTranslateService::new().await;

        // 簡単な英語から日本語への翻訳
        let result = service.translate("Hello World", "en", "ja").await;
        assert!(result.is_ok());
        let translated = result.unwrap();

        // 実際の翻訳結果を確認（「こんにちは世界」または類似の翻訳）
        assert!(!translated.is_empty());
        assert!(translated.contains("世界") || translated.contains("ワールド"));
    }

    #[tokio::test]
    async fn test_amazon_translate_japanese_to_english() {
        let service = AmazonTranslateService::new().await;

        // 日本語から英語への翻訳
        let result = service.translate("レイカーズがトレード", "ja", "en").await;
        assert!(result.is_ok());
        let translated = result.unwrap();

        // 実際の翻訳結果を確認
        assert!(!translated.is_empty());
        assert!(
            translated.to_lowercase().contains("lakers")
                || translated.to_lowercase().contains("trade")
        );
    }

    #[tokio::test]
    async fn test_amazon_translate_empty_text() {
        let service = AmazonTranslateService::new().await;

        // 空のテキストの処理を確認
        let result = service.translate("", "en", "ja").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");

        // 空白のみのテキスト
        let result = service.translate("   ", "en", "ja").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[tokio::test]
    async fn test_amazon_translate_with_special_characters() {
        let service = AmazonTranslateService::new().await;

        // 特殊文字を含むテキストの翻訳
        let text = "Lakers' \"Big Trade\" & More!";
        let result = service.translate(text, "en", "ja").await;
        assert!(result.is_ok());
        let translated = result.unwrap();

        // 翻訳結果が返されることを確認
        assert!(!translated.is_empty());
        assert!(translated.len() > 5); // 実際の翻訳が返される
    }

    #[tokio::test]
    async fn test_amazon_translate_long_text() {
        let service = AmazonTranslateService::new().await;

        // 長いテキストの翻訳
        let long_text = "The Los Angeles Lakers have acquired a star player in a blockbuster trade deal. This move is expected to significantly improve their championship chances.";
        let result = service.translate(long_text, "en", "ja").await;
        assert!(result.is_ok());
        let translated = result.unwrap();

        // 翻訳結果の検証
        assert!(!translated.is_empty());
        assert!(translated.contains("レイカーズ") || translated.contains("Lakers"));
        assert!(translated.len() > 50); // 実質的な翻訳が返される
    }

    #[tokio::test]
    async fn test_amazon_translate_service_initialization() {
        // 実際のAWS SDKの初期化テスト
        let service = AmazonTranslateService::new().await;

        // サービスが正しく初期化されたことを確認するため、簡単な翻訳を実行
        let result = service.translate("test", "en", "ja").await;
        // AWS認証が設定されていれば成功、なければエラー
        // どちらの場合も初期化自体は成功している
        assert!(result.is_ok() || matches!(result, Err(TranslationError::AwsError(_))));
    }

    #[tokio::test]
    async fn test_amazon_translate_invalid_language_code() {
        let service = AmazonTranslateService::new().await;

        // 無効な言語コードでエラーを発生させる
        let result = service.translate("Hello", "invalid", "ja").await;
        assert!(result.is_err());

        match result {
            Err(TranslationError::TranslationFailed(msg))
            | Err(TranslationError::AwsError(msg)) => {
                // AWS Translateからのエラーメッセージを確認
                assert!(!msg.is_empty());
            }
            _ => panic!("Expected translation error"),
        }
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

    #[tokio::test]
    async fn test_amazon_translate_html_content() {
        let service = AmazonTranslateService::new().await;

        // HTMLタグを含むテキストの翻訳
        let html_text = "<p>Lakers win the championship</p>";
        let result = service.translate(html_text, "en", "ja").await;
        assert!(result.is_ok());
        let translated = result.unwrap();

        // AWS TranslateはHTMLタグを保持するかどうか確認
        assert!(!translated.is_empty());
        // 翻訳結果に「レイカーズ」または「Lakers」が含まれる
        assert!(translated.contains("レイカーズ") || translated.contains("Lakers"));
    }

    #[tokio::test]
    async fn test_amazon_translate_multiple_sentences() {
        let service = AmazonTranslateService::new().await;

        // 複数の文を含むテキスト
        let text = "The Lakers won. The Celtics lost. It was a great game.";
        let result = service.translate(text, "en", "ja").await;
        assert!(result.is_ok());
        let translated = result.unwrap();

        // 翻訳が正しく完了したことを確認
        assert!(!translated.is_empty());
        assert!(translated.len() > 20); // 適切な長さの翻訳が返される
    }

    #[tokio::test]
    async fn test_amazon_translate_mixed_content() {
        let service = AmazonTranslateService::new().await;

        // 数字、記号、英語を含む混合コンテンツ
        let mixed_text = "Lakers scored 120 points! Amazing performance @ Staples Center.";
        let result = service.translate(mixed_text, "en", "ja").await;
        assert!(result.is_ok());
        let translated = result.unwrap();

        // 数字が保持されていることを確認
        assert!(translated.contains("120"));
        assert!(!translated.is_empty());
    }

    #[tokio::test]
    async fn test_amazon_translate_unicode_emoji() {
        let service = AmazonTranslateService::new().await;

        // 絵文字を含むテキスト
        let emoji_text = "Lakers won! 🏀 🏆";
        let result = service.translate(emoji_text, "en", "ja").await;
        assert!(result.is_ok());
        let translated = result.unwrap();

        // 翻訳が成功したことを確認
        assert!(!translated.is_empty());
        // 絵文字が保持されるか、または適切に処理される
        assert!(translated.len() > 5);
    }

    #[tokio::test]
    async fn test_amazon_translate_batch_requests() {
        let service = AmazonTranslateService::new().await;

        // 複数の翻訳リクエストを連続で実行
        let texts = vec![
            "Hello World",
            "Good morning",
            "Thank you",
            "See you later",
            "Welcome",
        ];

        for text in texts {
            let result = service.translate(text, "en", "ja").await;
            assert!(result.is_ok(), "Failed to translate: {}", text);
            let translated = result.unwrap();
            assert!(!translated.is_empty());

            // 連続リクエスト間に少し待機（レート制限回避）
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }

    #[tokio::test]
    async fn test_amazon_translate_trademark_symbols() {
        let service = AmazonTranslateService::new().await;

        // 商標記号を含むテキスト
        let special_text = "NBA™ announces new rules";
        let result = service.translate(special_text, "en", "ja").await;
        assert!(result.is_ok());
        let translated = result.unwrap();

        // 翻訳が成功したことを確認
        assert!(!translated.is_empty());
        assert!(translated.contains("NBA") || translated.contains("ＮＢＡ"));
    }

    #[tokio::test]
    async fn test_amazon_translate_japanese_mixed_content() {
        let service = AmazonTranslateService::new().await;

        // 日本語と英語が混在するテキスト
        let unicode_text = "レイカーズLakersが優勝";
        let result = service.translate(unicode_text, "ja", "en").await;
        assert!(result.is_ok());
        let translated = result.unwrap();

        // 翻訳結果に"Lakers"が含まれる
        assert!(!translated.is_empty());
        assert!(translated.contains("Lakers") || translated.contains("lakers"));
    }

    #[tokio::test]
    async fn test_translation_error_debug() {
        let error = TranslationError::AwsError("Debug test".to_string());
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("AwsError"));
        assert!(debug_str.contains("Debug test"));
    }

    #[tokio::test]
    async fn test_amazon_translate_html_entities() {
        let service = AmazonTranslateService::new().await;

        // HTMLエンティティを含むテキスト
        let html_text = "Lakers &amp; Celtics: Trade &quot;News&quot;";
        let result = service.translate(html_text, "en", "ja").await;
        assert!(result.is_ok());
        let translated = result.unwrap();

        // 翻訳が成功したことを確認
        assert!(!translated.is_empty());
        assert!(translated.contains("レイカーズ") || translated.contains("Lakers"));
        assert!(translated.contains("セルティックス") || translated.contains("Celtics"));
    }

    #[tokio::test]
    async fn test_amazon_translate_very_long_text() {
        let service = AmazonTranslateService::new().await;

        // AWS Translateの文字数制限に近い長いテキスト（5000文字以下）
        let long_paragraph =
            "The Los Angeles Lakers are one of the most successful basketball teams. ".repeat(50);
        let result = service.translate(&long_paragraph, "en", "ja").await;
        assert!(result.is_ok());
        let translated = result.unwrap();

        // 長いテキストが正しく翻訳される
        assert!(!translated.is_empty());
        assert!(translated.len() > 100); // 十分な長さの翻訳が返される
    }

    #[tokio::test]
    async fn test_amazon_translate_supported_languages() {
        let service = AmazonTranslateService::new().await;

        // AWS Translateがサポートする言語ペアをテスト
        let test_cases = vec![
            ("Hello", "en", "ja"),
            ("こんにちは", "ja", "en"),
            ("Hello", "en", "es"),
            ("Bonjour", "fr", "en"),
            ("Hallo", "de", "ja"),
        ];

        for (text, source, target) in test_cases {
            let result = service.translate(text, source, target).await;
            assert!(
                result.is_ok(),
                "Failed to translate {} from {} to {}",
                text,
                source,
                target
            );
            let translated = result.unwrap();
            assert!(!translated.is_empty());
        }
    }

    #[test]
    fn test_translation_service_trait_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<AmazonTranslateService>();
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
    async fn test_amazon_translate_concurrent_requests() {
        // 並行実行のテスト
        let texts = vec!["Hello", "World", "Test", "Concurrent"];

        let mut handles = vec![];

        for text in texts {
            let handle = tokio::spawn(async move {
                let service = AmazonTranslateService::new().await;
                service.translate(text, "en", "ja").await
            });
            handles.push(handle);
        }

        // 全ての翻訳が成功することを確認
        for handle in handles {
            let result = handle.await.unwrap();
            assert!(result.is_ok());
            let translated = result.unwrap();
            assert!(!translated.is_empty());
        }
    }

    #[test]
    fn test_translation_error_is_send_sync() {
        // TranslationErrorがSend + Syncであることを確認
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<TranslationError>();
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
}
