package components

import androidx.compose.runtime.*
import api.GraphQLClient
import kotlinx.coroutines.launch
import models.NewsItem
import org.jetbrains.compose.web.css.*
import org.jetbrains.compose.web.dom.*

@Composable
fun NewsList(selectedCategory: String?) {
    var newsItems by remember { mutableStateOf<List<NewsItem>>(emptyList()) }
    var isLoading by remember { mutableStateOf(true) }
    var error by remember { mutableStateOf<String?>(null) }
    
    val client = remember { GraphQLClient() }
    val scope = rememberCoroutineScope()
    
    LaunchedEffect(selectedCategory) {
        isLoading = true
        error = null
        scope.launch {
            try {
                newsItems = client.fetchNewsItems(selectedCategory)
                error = null
            } catch (e: Exception) {
                error = "ニュースの取得に失敗しました: ${e.message}"
                console.error("Failed to fetch news", e)
            } finally {
                isLoading = false
            }
        }
    }
    
    Div(attrs = {
        classes("news-list")
    }) {
        when {
            isLoading -> {
                Div(attrs = {
                    classes("loading-container")
                }) {
                    Div(attrs = {
                        classes("loading-spinner")
                    })
                    P { Text("ニュースを読み込んでいます...") }
                }
            }
            error != null -> {
                Div(attrs = {
                    classes("error-container")
                }) {
                    P(attrs = {
                        classes("error-message")
                    }) {
                        Text(error!!)
                    }
                }
            }
            newsItems.isEmpty() -> {
                Div(attrs = {
                    classes("empty-container")
                }) {
                    P { Text("ニュースが見つかりませんでした。") }
                }
            }
            else -> {
                newsItems.forEach { newsItem ->
                    NewsCard(newsItem)
                }
            }
        }
    }
    
    Style(NewsListStyles)
}

object NewsListStyles : StyleSheet() {
    init {
        ".news-list" style {
            marginTop(2.em)
        }
        
        ".loading-container" style {
            textAlign("center")
            padding(3.em)
        }
        
        ".loading-spinner" style {
            property("border", "4px solid #f3f3f3")
            property("border-top", "4px solid #1976d2")
            borderRadius(50.percent)
            width(40.px)
            height(40.px)
            property("animation", "spin 1s linear infinite")
            margin("0 auto 1em")
        }
        
        ".error-container" style {
            backgroundColor(Color("#ffebee"))
            border("1px solid #ef5350")
            borderRadius(4.px)
            padding(1.5.em)
            margin(1.em.unsafeCast<CSSMargin>())
        }
        
        ".error-message" style {
            color(Color("#c62828"))
            margin(0.px)
        }
        
        ".empty-container" style {
            textAlign("center")
            padding(3.em)
            color(Color("#666"))
        }
        
        media("(prefers-reduced-motion: no-preference)") {
            "keyframes spin" {
                0.percent {
                    property("transform", "rotate(0deg)")
                }
                100.percent {
                    property("transform", "rotate(360deg)")
                }
            }
        }
    }
}