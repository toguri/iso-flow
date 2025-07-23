import androidx.compose.runtime.*
import kotlinx.browser.window
import components.CategoryFilter
import components.NewsList
import models.NewsCategory
import org.jetbrains.compose.web.css.*
import org.jetbrains.compose.web.dom.*

@Composable
fun App() {
    var selectedCategory by remember { mutableStateOf(NewsCategory.ALL) }
    
    Style(AppStyles)
    
    Div(attrs = {
        classes("app-container")
    }) {
        // ヘッダー
        Header(attrs = {
            classes("app-header")
        }) {
            Div(attrs = {
                classes("header-content")
            }) {
                H1(attrs = {
                    classes("app-title")
                }) {
                    Text("NBA Trade Tracker")
                }
            }
        }
        
        // メインコンテンツ
        Main(attrs = {
            classes("main-content")
        }) {
            Div(attrs = {
                classes("content-wrapper")
            }) {
                // カテゴリーフィルター
                CategoryFilter(
                    selectedCategory = selectedCategory,
                    onCategoryChange = { selectedCategory = it }
                )
                
                // ニュース一覧
                NewsList(category = selectedCategory)
            }
        }
    }
}

object AppStyles : StyleSheet() {
    init {
        "*" style {
            margin(0.px)
            padding(0.px)
            boxSizing("border-box")
        }
        
        "body" style {
            property("font-family", "system-ui, -apple-system, sans-serif")
            backgroundColor(rgb(248, 249, 250))
        }
        
        ".app-container" style {
            minHeight(100.vh)
            display(DisplayStyle.Flex)
            flexDirection(FlexDirection.Column)
        }
        
        ".app-header" style {
            backgroundColor(rgb(29, 66, 138)) // NBA Blue
            color(Color.white)
            padding(16.px)
            property("box-shadow", "0 2px 4px rgba(0,0,0,0.1)")
        }
        
        ".header-content" style {
            maxWidth(1200.px)
            margin(0.px)
            property("margin", "0 auto")
        }
        
        ".app-title" style {
            fontSize(32.px)
            fontWeight(700)
        }
        
        ".main-content" style {
            flex(1)
            padding(32.px)
        }
        
        ".content-wrapper" style {
            maxWidth(1200.px)
            margin(0.px)
            property("margin", "0 auto")
        }
    }
}