package models

import kotlinx.serialization.Serializable

@Serializable
data class NewsItem(
    val id: String,
    val title: String,
    val description: String? = null,
    val link: String,
    val source: String,
    val category: String,
    val publishedAt: String
)

enum class NewsCategory(val value: String, val displayName: String) {
    ALL("all", "All"),
    TRADE("Trade", "Trade"),
    SIGNING("Signing", "Signing"),
    OTHER("Other", "Other");
    
    companion object {
        fun fromValue(value: String): NewsCategory {
            return values().find { it.value == value } ?: ALL
        }
    }
}