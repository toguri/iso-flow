package components

import androidx.compose.runtime.Composable
import models.NewsItem
import org.jetbrains.compose.web.css.*
import org.jetbrains.compose.web.dom.*
external fun Date(dateString: String): dynamic

fun formatDate(dateString: String): String {
    val date = Date(dateString)
    return date.toLocaleString("ja-JP")
}

fun stripHtml(html: String): String {
    // 簡易的なHTML除去
    return html
        .replace(Regex("<[^>]*>"), "") // HTMLタグを除去
        .replace(Regex("&nbsp;"), " ")
        .replace(Regex("&amp;"), "&")
        .replace(Regex("&lt;"), "<")
        .replace(Regex("&gt;"), ">")
        .replace(Regex("&quot;"), "\"")
        .trim()
}

@Composable
fun NewsCard(news: NewsItem) {
    Article(attrs = {
        classes("news-card")
        style {
            backgroundColor(Color.white)
            borderRadius(8.px)
            padding(24.px)
            property("box-shadow", "0 1px 3px rgba(0,0,0,0.1)")
            property("transition", "box-shadow 0.2s ease")
            // hover effect will be handled differently
        }
    }) {
        // カテゴリーバッジ
        Div(attrs = {
            style {
                display(DisplayStyle.Flex)
                justifyContent(JustifyContent.SpaceBetween)
                alignItems(AlignItems.Center)
                marginBottom(16.px)
            }
        }) {
            Span(attrs = {
                classes("category-badge")
                style {
                    padding(4.px, 12.px)
                    borderRadius(4.px)
                    fontSize(14.px)
                    fontWeight(600)
                    when (news.category) {
                        "Trade" -> {
                            backgroundColor(rgb(254, 226, 226))
                            color(rgb(185, 28, 28))
                        }
                        "Signing" -> {
                            backgroundColor(rgb(219, 234, 254))
                            color(rgb(29, 78, 216))
                        }
                        else -> {
                            backgroundColor(rgb(243, 244, 246))
                            color(rgb(75, 85, 99))
                        }
                    }
                }
            }) {
                Text(news.category)
            }
            
            Span(attrs = {
                style {
                    fontSize(14.px)
                    color(rgb(107, 114, 128))
                }
            }) {
                Text(news.source)
            }
        }
        
        // タイトル
        H3(attrs = {
            style {
                marginBottom(8.px)
                fontSize(20.px)
                fontWeight(600)
                property("line-height", "1.4")
            }
        }) {
            A(href = news.link, attrs = {
                attr("target", "_blank")
                style {
                    color(rgb(33, 33, 33))
                    textDecoration("none")
                        // hover effect
                }
            }) {
                Text(stripHtml(news.title))
            }
        }
        
        // 説明文
        news.description?.let { desc ->
            P(attrs = {
                style {
                    color(rgb(75, 85, 99))
                    marginBottom(16.px)
                    property("line-height", "1.6")
                }
            }) {
                Text(stripHtml(desc))
            }
        }
        
        // 日時
        Div(attrs = {
            style {
                fontSize(14.px)
                color(rgb(107, 114, 128))
            }
        }) {
            Text(formatDate(news.publishedAt))
        }
    }
}