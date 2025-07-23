package components

import androidx.compose.runtime.Composable
import models.NewsItem
import org.jetbrains.compose.web.css.*
import org.jetbrains.compose.web.dom.*
import kotlin.js.Date

@Composable
fun NewsCard(newsItem: NewsItem) {
    Article(attrs = {
        classes("news-card")
    }) {
        Div(attrs = {
            classes("news-card-header")
        }) {
            H3(attrs = {
                classes("news-card-title")
            }) {
                A(href = newsItem.link, attrs = {
                    target(ATarget.Blank)
                    attr("rel", "noopener noreferrer")
                }) {
                    Text(newsItem.title)
                }
            }
            Span(attrs = {
                classes("news-card-category", "category-${newsItem.category.lowercase()}")
            }) {
                Text(newsItem.category)
            }
        }
        
        P(attrs = {
            classes("news-card-description")
        }) {
            Text(newsItem.description)
        }
        
        Div(attrs = {
            classes("news-card-footer")
        }) {
            Span(attrs = {
                classes("news-card-date")
            }) {
                Text(formatDate(newsItem.pubDate))
            }
            Span(attrs = {
                classes("news-card-source")
            }) {
                Text("Source: ${extractDomain(newsItem.sourceUrl)}")
            }
        }
    }
    
    Style(NewsCardStyles)
}

fun formatDate(dateString: String): String {
    return try {
        val date = Date(dateString)
        val year = date.getFullYear()
        val month = (date.getMonth() + 1).toString().padStart(2, '0')
        val day = date.getDate().toString().padStart(2, '0')
        "$year.$month.$day 日本時間"
    } catch (e: Exception) {
        dateString
    }
}

fun extractDomain(url: String): String {
    return try {
        url.substringAfter("://").substringBefore("/")
    } catch (e: Exception) {
        url
    }
}

object NewsCardStyles : StyleSheet() {
    init {
        ".news-card" style {
            backgroundColor(Color.white)
            borderRadius(8.px)
            padding(1.5.em)
            marginBottom(1.5.em)
            boxShadow("0 2px 4px rgba(0,0,0,0.1)")
            property("transition", "box-shadow 0.3s ease")
        }
        
        ".news-card:hover" style {
            boxShadow("0 4px 8px rgba(0,0,0,0.15)")
        }
        
        ".news-card-header" style {
            display(DisplayStyle.Flex)
            justifyContent(JustifyContent.SpaceBetween)
            alignItems(AlignItems.Start)
            marginBottom(1.em)
        }
        
        ".news-card-title" style {
            flex(1)
            marginRight(1.em)
            fontSize(1.3.em)
            lineHeight(1.4)
        }
        
        ".news-card-title a" style {
            color(Color("#1976d2"))
            textDecoration("none")
            property("transition", "color 0.3s ease")
        }
        
        ".news-card-title a:hover" style {
            color(Color("#1565c0"))
            textDecoration("underline")
        }
        
        ".news-card-category" style {
            padding("0.3em 0.8em")
            borderRadius(4.px)
            fontSize(0.85.em)
            fontWeight(600)
            textTransform("uppercase")
            whiteSpace("nowrap")
        }
        
        ".category-technology" style {
            backgroundColor(Color("#e3f2fd"))
            color(Color("#1565c0"))
        }
        
        ".category-programming" style {
            backgroundColor(Color("#f3e5f5"))
            color(Color("#6a1b9a"))
        }
        
        ".category-ai" style {
            backgroundColor(Color("#e8f5e9"))
            color(Color("#2e7d32"))
        }
        
        ".category-web" style {
            backgroundColor(Color("#fff3e0"))
            color(Color("#e65100"))
        }
        
        ".category-mobile" style {
            backgroundColor(Color("#fce4ec"))
            color(Color("#c2185b"))
        }
        
        ".category-security" style {
            backgroundColor(Color("#ffebee"))
            color(Color("#c62828"))
        }
        
        ".category-cloud" style {
            backgroundColor(Color("#e0f2f1"))
            color(Color("#00695c"))
        }
        
        ".category-data" style {
            backgroundColor(Color("#f1f8e9"))
            color(Color("#558b2f"))
        }
        
        ".news-card-description" style {
            color(Color("#666"))
            lineHeight(1.6)
            marginBottom(1.em)
        }
        
        ".news-card-footer" style {
            display(DisplayStyle.Flex)
            justifyContent(JustifyContent.SpaceBetween)
            fontSize(0.85.em)
            color(Color("#999"))
        }
        
        ".news-card-date" style {
            fontWeight(500)
        }
        
        ".news-card-source" style {
            fontStyle("italic")
        }
    }
}