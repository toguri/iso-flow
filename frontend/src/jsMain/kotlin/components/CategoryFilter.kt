package components

import androidx.compose.runtime.Composable
import org.jetbrains.compose.web.attributes.selected
import org.jetbrains.compose.web.css.*
import org.jetbrains.compose.web.dom.*

@Composable
fun CategoryFilter(
    selectedCategory: String?,
    onCategorySelected: (String?) -> Unit
) {
    val categories = listOf(
        null to "すべて",
        "Trade" to "トレード",
        "Signing" to "契約・サイン",
        "Other" to "その他"
    )
    
    Div(attrs = {
        classes("category-filter")
    }) {
        Label(attrs = {
            classes("category-label")
        }) {
            Text("カテゴリを選択:")
        }
        
        Select(attrs = {
            classes("category-select")
            onChange { event ->
                val value = event.target.value
                onCategorySelected(if (value.isEmpty()) null else value)
            }
        }) {
            categories.forEach { (value, label) ->
                Option(
                    value = value ?: "",
                    attrs = {
                        if (selectedCategory == value) {
                            selected()
                        }
                    }
                ) {
                    Text(label)
                }
            }
        }
    }
    
    Style(CategoryFilterStyles)
}

object CategoryFilterStyles : StyleSheet() {
    init {
        ".category-filter" style {
            display(DisplayStyle.Flex)
            alignItems(AlignItems.Center)
            gap(1.em)
            padding(1.em)
            backgroundColor(Color.white)
            borderRadius(8.px)
            property("box-shadow", "0 2px 4px rgba(0,0,0,0.1)")
        }
        
        ".category-label" style {
            fontWeight(600)
            color(Color("#555"))
        }
        
        ".category-select" style {
            padding(0.5.em, 1.em)
            fontSize(1.em)
            property("border", "2px solid #ddd")
            borderRadius(4.px)
            backgroundColor(Color.white)
            cursor("pointer")
            property("transition", "border-color 0.3s ease")
            minWidth(200.px)
        }
        
        ".category-select:hover" style {
            property("border-color", "#1976d2")
        }
        
        ".category-select:focus" style {
            outline("none")
            property("border-color", "#1976d2")
            property("box-shadow", "0 0 0 3px rgba(25, 118, 210, 0.1)")
        }
        
        media("(max-width: 600px)") {
            ".category-filter" style {
                flexDirection(FlexDirection.Column)
                alignItems(AlignItems.Stretch)
            }
            
            ".category-select" style {
                width(100.percent)
            }
        }
    }
}