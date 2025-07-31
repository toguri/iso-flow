package models

import kotlinx.serialization.Serializable

@Serializable
data class NewsItem(
    val id: String,
    val title: String,
    val description: String? = null,
    val link: String,
    val source: String,
    val publishedAt: String,
    val category: String,
    val titleJa: String? = null,
    val descriptionJa: String? = null,
    val translationStatus: String = "pending",
    val translatedAt: String? = null
)