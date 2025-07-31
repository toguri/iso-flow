package models

import kotlinx.serialization.json.Json
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNull

class NewsItemTest {
    
    private val json = Json {
        ignoreUnknownKeys = true
    }
    
    @Test
    fun testNewsItemSerialization() {
        val newsItem = NewsItem(
            id = "123",
            title = "Test Trade News",
            description = "This is a test description",
            link = "https://example.com/news/123",
            source = "ESPN",
            publishedAt = "2025-07-27T12:00:00Z",
            category = "Trade",
            titleJa = null,
            descriptionJa = null,
            translationStatus = "pending",
            translatedAt = null
        )
        
        val jsonString = json.encodeToString(NewsItem.serializer(), newsItem)
        val decoded = json.decodeFromString(NewsItem.serializer(), jsonString)
        
        assertEquals(newsItem.id, decoded.id)
        assertEquals(newsItem.title, decoded.title)
        assertEquals(newsItem.description, decoded.description)
        assertEquals(newsItem.link, decoded.link)
        assertEquals(newsItem.source, decoded.source)
        assertEquals(newsItem.publishedAt, decoded.publishedAt)
        assertEquals(newsItem.category, decoded.category)
        assertEquals(newsItem.titleJa, decoded.titleJa)
        assertEquals(newsItem.descriptionJa, decoded.descriptionJa)
        assertEquals(newsItem.translationStatus, decoded.translationStatus)
        assertEquals(newsItem.translatedAt, decoded.translatedAt)
    }
    
    @Test
    fun testNewsItemWithNullDescription() {
        val newsItem = NewsItem(
            id = "456",
            title = "Breaking: Player Signed",
            description = null,
            link = "https://example.com/news/456",
            source = "RealGM",
            publishedAt = "2025-07-27T14:00:00Z",
            category = "Signing",
            titleJa = null,
            descriptionJa = null,
            translationStatus = "pending",
            translatedAt = null
        )
        
        assertNull(newsItem.description)
        
        val jsonString = json.encodeToString(NewsItem.serializer(), newsItem)
        val decoded = json.decodeFromString(NewsItem.serializer(), jsonString)
        
        assertNull(decoded.description)
        assertNull(decoded.titleJa)
        assertNull(decoded.descriptionJa)
        assertEquals("pending", decoded.translationStatus)
        assertNull(decoded.translatedAt)
    }
    
    @Test
    fun testNewsItemDeserialization() {
        val jsonString = """
            {
                "id": "789",
                "title": "NBA News Update",
                "description": "Latest NBA news",
                "link": "https://example.com/news/789",
                "source": "NBA.com",
                "publishedAt": "2025-07-27T16:00:00Z",
                "category": "Other",
                "titleJa": null,
                "descriptionJa": null,
                "translationStatus": "pending",
                "translatedAt": null
            }
        """.trimIndent()
        
        val newsItem = json.decodeFromString(NewsItem.serializer(), jsonString)
        
        assertEquals("789", newsItem.id)
        assertEquals("NBA News Update", newsItem.title)
        assertEquals("Latest NBA news", newsItem.description)
        assertEquals("https://example.com/news/789", newsItem.link)
        assertEquals("NBA.com", newsItem.source)
        assertEquals("2025-07-27T16:00:00Z", newsItem.publishedAt)
        assertEquals("Other", newsItem.category)
        assertNull(newsItem.titleJa)
        assertNull(newsItem.descriptionJa)
        assertEquals("pending", newsItem.translationStatus)
        assertNull(newsItem.translatedAt)
    }
    
    @Test
    fun testNewsItemDeserializationWithExtraFields() {
        // 未知のフィールドがあっても正常に動作することを確認
        val jsonString = """
            {
                "id": "999",
                "title": "Test Title",
                "link": "https://test.com",
                "source": "Test Source",
                "publishedAt": "2025-07-27T18:00:00Z",
                "category": "Trade",
                "extraField": "This should be ignored"
            }
        """.trimIndent()
        
        val newsItem = json.decodeFromString(NewsItem.serializer(), jsonString)
        
        assertEquals("999", newsItem.id)
        assertEquals("Test Title", newsItem.title)
        assertNull(newsItem.description) // descriptionフィールドがない場合はnull
        assertNull(newsItem.titleJa)
        assertNull(newsItem.descriptionJa)
        assertEquals("pending", newsItem.translationStatus)
        assertNull(newsItem.translatedAt)
    }
}