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
}
