package api

import kotlinx.browser.window
import kotlinx.coroutines.await
import kotlinx.serialization.Serializable
import kotlinx.serialization.decodeFromString
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import models.NewsItem
import org.w3c.fetch.*
import kotlin.js.json

@Serializable
data class GraphQLRequest(
    val query: String,
    val variables: Map<String, String>? = null
)

@Serializable
data class GraphQLResponse<T>(
    val data: T?,
    val errors: List<GraphQLError>? = null
)

@Serializable
data class GraphQLError(
    val message: String
)

@Serializable
data class TradeNewsData(
    val tradeNews: List<NewsItem>
)

@Serializable
data class TradeNewsByCategoryData(
    val tradeNewsByCategory: List<NewsItem>
)

object GraphQLClient {
    private const val GRAPHQL_URL = "http://localhost:8000"
    
    private val json = Json {
        ignoreUnknownKeys = true
        isLenient = true
    }
    
    suspend fun fetchAllNews(): List<NewsItem> {
        val query = """
            query {
                tradeNews {
                    id
                    title
                    description
                    link
                    source
                    category
                    publishedAt
                }
            }
        """.trimIndent()
        
        val request = GraphQLRequest(query)
        val response = executeQuery<TradeNewsData>(request)
        
        if (response.errors != null) {
            throw Exception(response.errors.first().message)
        }
        
        return response.data?.tradeNews ?: emptyList()
    }
    
    suspend fun fetchNewsByCategory(category: String): List<NewsItem> {
        val query = """
            query GetNewsByCategory(${'$'}category: String!) {
                tradeNewsByCategory(category: ${'$'}category) {
                    id
                    title
                    description
                    link
                    source
                    category
                    publishedAt
                }
            }
        """.trimIndent()
        
        val request = GraphQLRequest(
            query = query,
            variables = mapOf("category" to category)
        )
        val response = executeQuery<TradeNewsByCategoryData>(request)
        
        if (response.errors != null) {
            throw Exception(response.errors.first().message)
        }
        
        return response.data?.tradeNewsByCategory ?: emptyList()
    }
    
    private suspend inline fun <reified T> executeQuery(request: GraphQLRequest): GraphQLResponse<T> {
        val response = window.fetch(
            GRAPHQL_URL,
            RequestInit(
                method = "POST",
                headers = js("{'Content-Type': 'application/json'}"),
                body = json.encodeToString(request)
            )
        ).await()
        
        if (!response.ok) {
            throw Exception("HTTP error! status: ${response.status}")
        }
        
        val text = response.text().await()
        return json.decodeFromString(text)
    }
}