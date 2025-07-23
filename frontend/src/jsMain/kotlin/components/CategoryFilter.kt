package components

import androidx.compose.runtime.Composable
import models.NewsCategory
import org.jetbrains.compose.web.css.*
import org.jetbrains.compose.web.dom.*

@Composable
fun CategoryFilter(
    selectedCategory: NewsCategory,
    onCategoryChange: (NewsCategory) -> Unit
) {
    Div(attrs = {
        classes("category-filter")
        style {
            display(DisplayStyle.Flex)
            gap(16.px)
            marginBottom(32.px)
        }
    }) {
        NewsCategory.values().forEach { category ->
            Button(attrs = {
                if (selectedCategory == category) {
                    classes("category-button", "active")
                } else {
                    classes("category-button")
                }
                onClick {
                    onCategoryChange(category)
                }
                style {
                    padding(8.px, 16.px)
                    border(1.px, LineStyle.Solid, 
                        if (selectedCategory == category) rgb(29, 66, 138) else rgb(200, 200, 200)
                    )
                    backgroundColor(
                        if (selectedCategory == category) rgb(29, 66, 138) else Color.white
                    )
                    color(
                        if (selectedCategory == category) Color.white else rgb(33, 33, 33)
                    )
                    borderRadius(4.px)
                    cursor("pointer")
                    fontSize(16.px)
                    fontWeight(500)
                    property("transition", "all 0.2s ease")
                }
            }) {
                Text(category.displayName)
            }
        }
    }
}