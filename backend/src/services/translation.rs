//! 翻訳サービスの実装
//!
//! LibreTranslate APIを使用して英語から日本語への翻訳を提供します。

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, error, info};

#[derive(Error, Debug)]
pub enum TranslationError {
    #[error("API request failed: {0}")]
    ApiError(String),
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
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

#[derive(Serialize)]
struct TranslateRequest {
    q: String,
    source: String,
    target: String,
    format: String,
    api_key: Option<String>,
}

#[derive(Deserialize)]
struct TranslateResponse {
    #[serde(rename = "translatedText")]
    translated_text: String,
}

pub struct LibreTranslateService {
    client: Client,
    api_url: String,
    api_key: Option<String>,
}

impl LibreTranslateService {
    pub fn new(api_url: String, api_key: Option<String>) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            api_url,
            api_key,
        }
    }
}

#[async_trait]
impl TranslationService for LibreTranslateService {
    async fn translate(
        &self,
        text: &str,
        source_lang: &str,
        target_lang: &str,
    ) -> Result<String, TranslationError> {
        if text.trim().is_empty() {
            return Ok(String::new());
        }

        let request = TranslateRequest {
            q: text.to_string(),
            source: source_lang.to_string(),
            target: target_lang.to_string(),
            format: "text".to_string(),
            api_key: self.api_key.clone(),
        };

        debug!(
            "Translating text from {} to {}: {} chars",
            source_lang,
            target_lang,
            text.len()
        );

        let response = self
            .client
            .post(format!("{}/translate", self.api_url))
            .json(&request)
            .send()
            .await?;

        if response.status() == 429 {
            error!("Rate limit exceeded for translation API");
            return Err(TranslationError::RateLimitExceeded);
        }

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Translation API error: {} - {}", status, error_text);
            return Err(TranslationError::ApiError(format!(
                "HTTP {status}: {error_text}"
            )));
        }

        let translated: TranslateResponse = response.json().await?;
        info!(
            "Successfully translated {} chars to {}",
            text.len(),
            translated.translated_text.len()
        );

        Ok(translated.translated_text)
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
