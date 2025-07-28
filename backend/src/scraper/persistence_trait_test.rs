//! persistence_traitの防御的テスト

#[cfg(test)]
mod tests {
    use crate::scraper::persistence_trait::{mock::MockNewsPersistence, NewsPersistenceTrait, SavedNewsItem};
    use crate::scraper::{NewsItem, NewsSource};
    use chrono::Utc;

    #[tokio::test]
    async fn test_mock_save_success() {
        let mock = MockNewsPersistence::new();
        
        let items = vec![
            NewsItem {
                id: "test-1".to_string(),
                title: "Trade News 1".to_string(),
                description: Some("Description 1".to_string()),
                link: "https://example.com/1".to_string(),
                source: NewsSource::ESPN,
                published_at: Utc::now(),
                category: "Trade".to_string(),
            },
        ];

        let result = mock.save_news_items(items).await.unwrap();
        assert_eq!(result.saved_count, 1);
        assert_eq!(result.skipped_count, 0);
        assert_eq!(result.errors.len(), 0);
    }

    #[tokio::test]
    async fn test_mock_save_with_duplicates() {
        let mock = MockNewsPersistence::new();
        
        // 既存のアイテムを追加
        mock.add_existing_item(
            "existing-1".to_string(),
            SavedNewsItem {
                id: Some(1),
                external_id: "existing-1".to_string(),
                title: "Existing News".to_string(),
                description: None,
                source_name: "ESPN".to_string(),
                source_url: "https://example.com/existing".to_string(),
                category: "Trade".to_string(),
                is_official: Some(false),
                published_at: Utc::now().to_rfc3339(),
                scraped_at: None,
                created_at: None,
            },
        );

        let items = vec![
            NewsItem {
                id: "existing-1".to_string(), // 重複
                title: "Duplicate News".to_string(),
                description: None,
                link: "https://example.com/dup".to_string(),
                source: NewsSource::ESPN,
                published_at: Utc::now(),
                category: "Trade".to_string(),
            },
            NewsItem {
                id: "new-1".to_string(), // 新規
                title: "New News".to_string(),
                description: None,
                link: "https://example.com/new".to_string(),
                source: NewsSource::RealGM,
                published_at: Utc::now(),
                category: "Signing".to_string(),
            },
        ];

        let result = mock.save_news_items(items).await.unwrap();
        assert_eq!(result.saved_count, 1); // 新規のみ
        assert_eq!(result.skipped_count, 1); // 重複
        assert_eq!(result.errors.len(), 0);
    }

    #[tokio::test]
    async fn test_mock_save_with_specific_failures() {
        let mock = MockNewsPersistence::new();
        
        // 特定のIDで失敗するように設定
        mock.fail_on_id("fail-1".to_string());
        mock.fail_on_id("fail-2".to_string());

        let items = vec![
            NewsItem {
                id: "success-1".to_string(),
                title: "Success News".to_string(),
                description: None,
                link: "https://example.com/success".to_string(),
                source: NewsSource::ESPN,
                published_at: Utc::now(),
                category: "Trade".to_string(),
            },
            NewsItem {
                id: "fail-1".to_string(), // 失敗する
                title: "Fail News 1".to_string(),
                description: None,
                link: "https://example.com/fail1".to_string(),
                source: NewsSource::ESPN,
                published_at: Utc::now(),
                category: "Trade".to_string(),
            },
            NewsItem {
                id: "fail-2".to_string(), // 失敗する
                title: "Fail News 2".to_string(),
                description: None,
                link: "https://example.com/fail2".to_string(),
                source: NewsSource::RealGM,
                published_at: Utc::now(),
                category: "Signing".to_string(),
            },
        ];

        let result = mock.save_news_items(items).await.unwrap();
        assert_eq!(result.saved_count, 1); // 成功した1件
        assert_eq!(result.skipped_count, 0);
        assert_eq!(result.errors.len(), 2); // 失敗した2件
        assert!(result.errors.iter().any(|(id, _)| id == "fail-1"));
        assert!(result.errors.iter().any(|(id, _)| id == "fail-2"));
    }

    #[tokio::test]
    async fn test_mock_database_failure() {
        let mock = MockNewsPersistence::new();
        
        // すべての操作を失敗させる
        mock.set_should_fail(true);

        let items = vec![
            NewsItem {
                id: "test-1".to_string(),
                title: "Test News".to_string(),
                description: None,
                link: "https://example.com/test".to_string(),
                source: NewsSource::ESPN,
                published_at: Utc::now(),
                category: "Trade".to_string(),
            },
        ];

        // データベースエラーが発生することを確認
        let result = mock.save_news_items(items).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database connection error"));
    }

    #[tokio::test]
    async fn test_mock_get_recent_news() {
        let mock = MockNewsPersistence::new();
        
        // テストデータを追加
        for i in 0..10 {
            let item = SavedNewsItem {
                id: Some(i),
                external_id: format!("news-{}", i),
                title: format!("News {}", i),
                description: None,
                source_name: "ESPN".to_string(),
                source_url: format!("https://example.com/{}", i),
                category: "Trade".to_string(),
                is_official: Some(false),
                published_at: Utc::now().to_rfc3339(),
                scraped_at: None,
                created_at: None,
            };
            mock.add_existing_item(format!("news-{}", i), item);
        }

        // 最新5件を取得
        let recent = mock.get_recent_news(5).await.unwrap();
        assert_eq!(recent.len(), 5);
    }

    #[tokio::test]
    async fn test_mock_get_news_by_category() {
        let mock = MockNewsPersistence::new();
        
        // 異なるカテゴリのデータを追加
        let categories = vec!["Trade", "Signing", "Other"];
        for (i, category) in categories.iter().enumerate() {
            for j in 0..3 {
                let item = SavedNewsItem {
                    id: Some((i * 3 + j) as i64),
                    external_id: format!("{}-{}", category, j),
                    title: format!("{} News {}", category, j),
                    description: None,
                    source_name: "ESPN".to_string(),
                    source_url: format!("https://example.com/{}-{}", category, j),
                    category: category.to_string(),
                    is_official: Some(false),
                    published_at: Utc::now().to_rfc3339(),
                    scraped_at: None,
                    created_at: None,
                };
                mock.add_existing_item(format!("{}-{}", category, j), item);
            }
        }

        // カテゴリ別に取得
        let trade_news = mock.get_news_by_category("Trade").await.unwrap();
        assert_eq!(trade_news.len(), 3);
        assert!(trade_news.iter().all(|item| item.category == "Trade"));

        let signing_news = mock.get_news_by_category("Signing").await.unwrap();
        assert_eq!(signing_news.len(), 3);
        assert!(signing_news.iter().all(|item| item.category == "Signing"));

        let other_news = mock.get_news_by_category("Other").await.unwrap();
        assert_eq!(other_news.len(), 3);
        assert!(other_news.iter().all(|item| item.category == "Other"));

        // 存在しないカテゴリ
        let unknown = mock.get_news_by_category("Unknown").await.unwrap();
        assert_eq!(unknown.len(), 0);
    }

    #[tokio::test]
    async fn test_mock_exists_by_external_id() {
        let mock = MockNewsPersistence::new();
        
        // 既存のアイテムを追加
        mock.add_existing_item(
            "existing-id".to_string(),
            SavedNewsItem {
                id: Some(1),
                external_id: "existing-id".to_string(),
                title: "Existing".to_string(),
                description: None,
                source_name: "ESPN".to_string(),
                source_url: "https://example.com".to_string(),
                category: "Trade".to_string(),
                is_official: Some(false),
                published_at: Utc::now().to_rfc3339(),
                scraped_at: None,
                created_at: None,
            },
        );

        // 存在確認
        assert!(mock.exists_by_external_id("existing-id").await.unwrap());
        assert!(!mock.exists_by_external_id("non-existing-id").await.unwrap());
    }

    #[tokio::test]
    async fn test_mock_error_handling_on_query() {
        let mock = MockNewsPersistence::new();
        
        // エラーを発生させる
        mock.set_should_fail(true);

        // すべてのクエリがエラーを返すことを確認
        assert!(mock.exists_by_external_id("any-id").await.is_err());
        assert!(mock.get_recent_news(10).await.is_err());
        assert!(mock.get_news_by_category("Trade").await.is_err());
    }
}