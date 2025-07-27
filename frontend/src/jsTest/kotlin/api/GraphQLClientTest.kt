package api

import io.ktor.client.*
import io.ktor.client.engine.mock.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import io.ktor.utils.io.*
import kotlinx.coroutines.test.runTest
import kotlinx.serialization.json.Json
import models.NewsItem
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class GraphQLClientTest {
    
    private fun createMockClient(mockEngine: MockEngine): HttpClient {
        return HttpClient(mockEngine) {
            install(ContentNegotiation) {
                json(Json {
                    prettyPrint = true
                    isLenient = true
                    ignoreUnknownKeys = true
                })
            }
        }
    }
    
    @Test
    fun testFetchAllTradeNews() = runTest {
        val mockNewsItems = listOf(
            NewsItem(
                id = "1",
                title = "Test Trade 1",
                description = "Description 1",
                link = "https://example.com/1",
                source = "ESPN",
                publishedAt = "2025-07-27T12:00:00Z",
                category = "Trade"
            ),
            NewsItem(
                id = "2",
                title = "Test Signing",
                description = null,
                link = "https://example.com/2",
                source = "RealGM",
                publishedAt = "2025-07-27T13:00:00Z",
                category = "Signing"
            )
        )
        
        val mockEngine = MockEngine { request ->
            assertEquals("http://localhost:8000/graphql", request.url.toString())
            assertEquals(HttpMethod.Post, request.method)
            
            val responseData = GraphQLResponse(
                data = TradeNewsData(tradeNews = mockNewsItems)
            )
            
            respond(
                content = ByteReadChannel(Json.encodeToString(
                    GraphQLResponse.serializer(TradeNewsData.serializer()),
                    responseData
                )),
                status = HttpStatusCode.OK,
                headers = headersOf(HttpHeaders.ContentType, "application/json")
            )
        }
        
        // 現在のGraphQLClientの実装では直接テストが困難なため、
        // リクエスト/レスポンスのシリアライゼーションのみをテスト
        // 将来的にはGraphQLClientをリファクタリングして依存性注入を可能にする必要がある
    }
    
    @Test
    fun testGraphQLRequestSerialization() = runTest {
        val request = GraphQLRequest(
            query = "query { tradeNews { id title } }",
            variables = mapOf("category" to "Trade")
        )
        
        val json = Json { prettyPrint = true }
        val serialized = json.encodeToString(GraphQLRequest.serializer(), request)
        
        assertTrue(serialized.contains("query"))
        assertTrue(serialized.contains("variables"))
        assertTrue(serialized.contains("Trade"))
    }
    
    @Test
    fun testGraphQLResponseDeserialization() = runTest {
        val jsonResponse = """
            {
                "data": {
                    "tradeNews": [
                        {
                            "id": "123",
                            "title": "Breaking Trade News",
                            "description": "Major trade announced",
                            "link": "https://nba.com/news/123",
                            "source": "NBA.com",
                            "publishedAt": "2025-07-27T10:00:00Z",
                            "category": "Trade"
                        }
                    ]
                }
            }
        """.trimIndent()
        
        val response = Json.decodeFromString(
            GraphQLResponse.serializer(TradeNewsData.serializer()),
            jsonResponse
        )
        
        assertEquals(1, response.data?.tradeNews?.size)
        assertEquals("123", response.data?.tradeNews?.first()?.id)
        assertEquals("Breaking Trade News", response.data?.tradeNews?.first()?.title)
    }
    
    @Test
    fun testGraphQLErrorResponse() = runTest {
        val errorResponse = """
            {
                "data": null,
                "errors": [
                    {
                        "message": "Network error"
                    }
                ]
            }
        """.trimIndent()
        
        val response = Json.decodeFromString(
            GraphQLResponse.serializer(TradeNewsData.serializer()),
            errorResponse
        )
        
        assertTrue(response.data == null)
        assertEquals(1, response.errors?.size)
        assertEquals("Network error", response.errors?.first()?.message)
    }
}