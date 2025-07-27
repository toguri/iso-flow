package api

import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.engine.js.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.request.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import models.NewsItem

@Serializable
data class GraphQLRequest(
    val query: String,
    val variables: Map<String, String?> = emptyMap()
)

@Serializable
data class GraphQLResponse<T>(
    val data: T? = null,
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

class GraphQLClient {
    private val client = HttpClient(Js) {
        install(ContentNegotiation) {
            json(Json {
                prettyPrint = true
                isLenient = true
                ignoreUnknownKeys = true
            })
        }
    }
    
    private val endpoint = "http://localhost:8000/"
    
    suspend fun fetchNewsItems(category: String? = null): List<NewsItem> {
        return if (category == null) {
            fetchAllTradeNews()
        } else {
            fetchTradeNewsByCategory(category)
        }
    }
    
    private suspend fun fetchAllTradeNews(): List<NewsItem> {
        val query = """
            query GetAllTradeNews {
                tradeNews {
                    id
                    title
                    description
                    link
                    source
                    publishedAt
                    category
                }
            }
        """.trimIndent()
        
        val request = GraphQLRequest(query = query)
        
        return try {
            val response: GraphQLResponse<TradeNewsData> = client.post(endpoint) {
                contentType(ContentType.Application.Json)
                setBody(request)
            }.body()
            
            response.data?.tradeNews ?: emptyList()
        } catch (e: Exception) {
            console.error("Failed to fetch trade news", e)
            emptyList()
        }
    }
    
    private suspend fun fetchTradeNewsByCategory(category: String): List<NewsItem> {
        val query = """
            query GetTradeNewsByCategory(${'$'}category: String!) {
                tradeNewsByCategory(category: ${'$'}category) {
                    id
                    title
                    description
                    link
                    source
                    publishedAt
                    category
                }
            }
        """.trimIndent()
        
        val request = GraphQLRequest(
            query = query,
            variables = mapOf("category" to category)
        )
        
        return try {
            val response: GraphQLResponse<TradeNewsByCategoryData> = client.post(endpoint) {
                contentType(ContentType.Application.Json)
                setBody(request)
            }.body()
            
            response.data?.tradeNewsByCategory ?: emptyList()
        } catch (e: Exception) {
            console.error("Failed to fetch trade news by category", e)
            emptyList()
        }
    }
}