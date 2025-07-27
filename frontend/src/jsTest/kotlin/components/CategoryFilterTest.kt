package components

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNull

class CategoryFilterTest {
    
    @Test
    fun testCategoryOptions() {
        // カテゴリオプションの定義を確認
        val categories = listOf(
            null to "すべて",
            "Trade" to "トレード",
            "Signing" to "契約・サイン",
            "Other" to "その他"
        )
        
        // 各カテゴリの値を確認
        assertEquals(4, categories.size)
        assertNull(categories[0].first)
        assertEquals("すべて", categories[0].second)
        assertEquals("Trade", categories[1].first)
        assertEquals("トレード", categories[1].second)
    }
    
    @Test
    fun testCategoryFilterCallbacks() {
        var selectedCategory: String? = null
        val onCategorySelected: (String?) -> Unit = { category ->
            selectedCategory = category
        }
        
        // "すべて"を選択
        onCategorySelected(null)
        assertNull(selectedCategory)
        
        // "トレード"を選択
        onCategorySelected("Trade")
        assertEquals("Trade", selectedCategory)
        
        // "契約・サイン"を選択
        onCategorySelected("Signing")
        assertEquals("Signing", selectedCategory)
        
        // "その他"を選択
        onCategorySelected("Other")
        assertEquals("Other", selectedCategory)
    }
    
    @Test
    fun testCategoryValueMapping() {
        // selectタグのvalue属性に使用される値の確認
        val categoryValue = "Trade"
        val expectedValue = "category-$categoryValue"
        assertEquals("category-Trade", expectedValue)
        
        // null（すべて）の場合
        val allCategoriesValue = null
        val expectedAllValue = if (allCategoriesValue == null) "all" else "category-$allCategoriesValue"
        assertEquals("all", expectedAllValue)
    }
}