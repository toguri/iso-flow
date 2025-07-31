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
    async fn test_empty_text_translation() {
        let service = MockTranslationService;
        let result = service.translate("", "en", "ja").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "[翻訳済み] ");
    }
}
