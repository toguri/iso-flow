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
data class NewsItemsData(
    val newsItems: List<NewsItem>
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
    
    private val endpoint = "http://localhost:8080/graphql"
    
    suspend fun fetchNewsItems(category: String? = null): List<NewsItem> {
        val query = """
            query GetNewsItems(${'$'}category: String) {
                newsItems(category: ${'$'}category) {
                    id
                    title
                    description
                    link
                    pubDate
                    category
                    sourceUrl
                    createdAt
                    updatedAt
                }
            }
        """.trimIndent()
        
        val request = GraphQLRequest(
            query = query,
            variables = mapOf("category" to category)
        )
        
        return try {
            val response: GraphQLResponse<NewsItemsData> = client.post(endpoint) {
                contentType(ContentType.Application.Json)
                setBody(request)
            }.body()
            
            response.data?.newsItems ?: emptyList()
        } catch (e: Exception) {
            console.error("Failed to fetch news items", e)
            emptyList()
        }
    }
}