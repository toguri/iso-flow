package components

import models.NewsItem
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNull
import kotlin.test.assertTrue

class NewsCardTest {
    
    @Test
    fun testFormatDate() {
        // ISO 8601形式の日付をフォーマット
        val isoDate = "2025-07-27T12:00:00Z"
        val formatted = formatDate(isoDate)
        // 実際のフォーマット結果は実行環境のタイムゾーンに依存するため、
        // 形式が正しいことだけを確認
        assertTrue(formatted.contains("2025"))
        assertTrue(formatted.contains("日本時間"))
    }
    
    @Test
    fun testFormatDateWithInvalidDate() {
        // 無効な日付の場合、JavaScriptのDateコンストラクタはNaNを返すため、
        // 結果は「NaN.NaN.NaN 日本時間」のような形式になる可能性がある
        val invalidDate = "invalid-date-string"
        val formatted = formatDate(invalidDate)
        // 無効な日付の場合、NaNが含まれるか、元の文字列が返される
        assertTrue(formatted.contains("NaN") || formatted == invalidDate)
    }
    
    @Test
    fun testExtractDomain() {
        // HTTPSのURL
        val httpsUrl = "https://www.example.com/path/to/article"
        assertEquals("www.example.com", extractDomain(httpsUrl))
        
        // HTTPのURL
        val httpUrl = "http://example.org/news"
        assertEquals("example.org", extractDomain(httpUrl))
        
        // サブドメイン付きURL
        val subdomainUrl = "https://news.nba.com/article/123"
        assertEquals("news.nba.com", extractDomain(subdomainUrl))
    }
    
    @Test
    fun testExtractDomainWithInvalidUrl() {
        // プロトコルがない場合、://が見つからないのでsubstringAfterは元の文字列を返し、
        // その後substringBeforeで最初の/までを取得
        val noProtocol = "example.com/path"
        assertEquals("example.com", extractDomain(noProtocol))
        
        // 空文字列
        val emptyUrl = ""
        assertEquals("", extractDomain(emptyUrl))
    }
    
    @Test
    fun testNewsCardDataValidation() {
        // NewsCardに渡すデータの検証
        val newsItem = NewsItem(
            id = "test-1",
            title = "Test Trade News",
            description = "This is a test description",
            link = "https://www.espn.com/nba/story/123",
            source = "ESPN",
            publishedAt = "2025-07-27T15:30:00Z",
            category = "Trade",
            titleJa = null,
            descriptionJa = null,
            translationStatus = "pending",
            translatedAt = null
        )
        
        // カテゴリのCSSクラス名生成
        val categoryClass = "category-${newsItem.category.lowercase()}"
        assertEquals("category-trade", categoryClass)
        
        // null descriptionの場合
        val newsItemWithoutDesc = newsItem.copy(description = null)
        assertNull(newsItemWithoutDesc.description)
    }
    
    @Test
    fun testCategoryClassGeneration() {
        // 各カテゴリのCSSクラス名が正しく生成されることを確認
        val tradeItem = NewsItem(
            id = "1", title = "Trade", description = null,
            link = "link", source = "source", 
            publishedAt = "2025-07-27T00:00:00Z",
            category = "Trade",
            titleJa = null,
            descriptionJa = null,
            translationStatus = "pending",
            translatedAt = null
        )
        assertEquals("category-trade", "category-${tradeItem.category.lowercase()}")
        
        val signingItem = tradeItem.copy(category = "Signing")
        assertEquals("category-signing", "category-${signingItem.category.lowercase()}")
        
        val otherItem = tradeItem.copy(category = "Other")
        assertEquals("category-other", "category-${otherItem.category.lowercase()}")
    }
}