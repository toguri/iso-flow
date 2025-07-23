package models

import kotlinx.serialization.Serializable

@Serializable
data class NewsItem(
    val id: String,
    val title: String,
    val description: String,
    val link: String,
    val pubDate: String,
    val category: String,
    val sourceUrl: String,
    val createdAt: String,
    val updatedAt: String
)