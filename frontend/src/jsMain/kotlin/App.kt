import androidx.compose.runtime.*
import components.CategoryFilter
import components.NewsList
import org.jetbrains.compose.web.css.*
import org.jetbrains.compose.web.dom.*

@Composable
fun App() {
    var selectedCategory by remember { mutableStateOf<String?>(null) }
    
    Style(AppStylesheet)
    
    Div(attrs = {
        classes("app-container")
    }) {
        Header(attrs = {
            classes("app-header")
        }) {
            H1 {
                Text("ISO-Flow Tech News")
            }
            P(attrs = {
                classes("app-subtitle")
            }) {
                Text("最新のテクノロジーニュースをお届けします")
            }
        }
        
        Div(attrs = {
            classes("app-content")
        }) {
            CategoryFilter(
                selectedCategory = selectedCategory,
                onCategorySelected = { selectedCategory = it }
            )
            
            NewsList(selectedCategory = selectedCategory)
        }
        
        Footer(attrs = {
            classes("app-footer")
        }) {
            P {
                Text("© 2023 ISO-Flow Tech News. All rights reserved.")
            }
        }
    }
}

object AppStylesheet : StyleSheet() {
    init {
        "*" style {
            boxSizing("border-box")
            margin(0.px)
            padding(0.px)
        }
        
        "body" style {
            fontFamily("'Noto Sans JP', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif")
            backgroundColor(Color("#f5f5f5"))
            color(Color("#333"))
            lineHeight(1.6)
        }
        
        ".app-container" style {
            minHeight(100.vh)
            display(DisplayStyle.Flex)
            flexDirection(FlexDirection.Column)
        }
        
        ".app-header" style {
            backgroundColor(Color("#1976d2"))
            color(Color.white)
            padding(2.em)
            textAlign("center")
            boxShadow("0 2px 4px rgba(0,0,0,0.1)")
        }
        
        ".app-header h1" style {
            fontSize(2.5.em)
            marginBottom(0.2.em)
            fontWeight(700)
        }
        
        ".app-subtitle" style {
            fontSize(1.1.em)
            opacity(0.9)
        }
        
        ".app-content" style {
            flex(1)
            padding(2.em)
            maxWidth(1200.px)
            width(100.percent)
            margin("0 auto")
        }
        
        ".app-footer" style {
            backgroundColor(Color("#333"))
            color(Color.white)
            padding(1.5.em)
            textAlign("center")
            marginTop("auto")
        }
        
        ".app-footer p" style {
            margin(0.px)
        }
    }
}