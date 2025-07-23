package components

import androidx.compose.runtime.*
import api.GraphQLClient
import kotlinx.coroutines.launch
import models.NewsCategory
import models.NewsItem
import org.jetbrains.compose.web.css.*
import org.jetbrains.compose.web.dom.*

@Composable
fun NewsList(category: NewsCategory) {
    var newsList by remember { mutableStateOf<List<NewsItem>>(emptyList()) }
    var isLoading by remember { mutableStateOf(true) }
    var error by remember { mutableStateOf<String?>(null) }
    val scope = rememberCoroutineScope()
    
    // データ取得
    LaunchedEffect(category) {
        isLoading = true
        error = null
        
        try {
            val data = if (category == NewsCategory.ALL) {
                GraphQLClient.fetchAllNews()
            } else {
                GraphQLClient.fetchNewsByCategory(category.value)
            }
            newsList = data
        } catch (e: Exception) {
            error = e.message
        } finally {
            isLoading = false
        }
    }
    
    Div(attrs = {
        classes("news-list")
        style {
            display(DisplayStyle.Grid)
            property("grid-template-columns", "repeat(auto-fill, minmax(350px, 1fr))")
            gap(16.px)
        }
    }) {
        // ローディング表示
        if (isLoading) {
            Div(attrs = {
                style {
                    textAlign("center")
                    padding(32.px)
                }
            }) {
                Text("読み込み中...")
            }
        }
        
        // エラー表示
        error?.let { errorMessage ->
            Div(attrs = {
                style {
                    backgroundColor(rgb(254, 226, 226))
                    color(rgb(185, 28, 28))
                    padding(16.px)
                    borderRadius(4.px)
                    marginBottom(16.px)
                }
            }) {
                Text("エラー: $errorMessage")
            }
        }
        
        // ニュース一覧
        if (!isLoading && error == null) {
            if (newsList.isEmpty()) {
                Div(attrs = {
                    style {
                        textAlign("center")
                        padding(32.px)
                        color(rgb(107, 114, 128))
                    }
                }) {
                    Text("ニュースがありません")
                }
            } else {
                newsList.forEach { news ->
                    NewsCard(news)
                }
                
                // 更新ボタン
                Div(attrs = {
                    style {
                        textAlign("center")
                        marginTop(32.px)
                    }
                }) {
                    Button(attrs = {
                        onClick {
                            scope.launch {
                                isLoading = true
                                try {
                                    val data = if (category == NewsCategory.ALL) {
                                        GraphQLClient.fetchAllNews()
                                    } else {
                                        GraphQLClient.fetchNewsByCategory(category.value)
                                    }
                                    newsList = data
                                } catch (e: Exception) {
                                    error = e.message
                                } finally {
                                    isLoading = false
                                }
                            }
                        }
                        style {
                            backgroundColor(rgb(29, 66, 138))
                            color(Color.white)
                            padding(12.px, 24.px)
                            borderRadius(4.px)
                            border(0.px)
                            fontSize(16.px)
                            fontWeight(500)
                            cursor("pointer")
                            property("transition", "background-color 0.2s ease")
                            // hover effect
                        }
                    }) {
                        Text("更新")
                    }
                }
            }
        }
    }
}