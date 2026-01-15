// ============================================================================ 
// 集成测试 (Integration Tests)
// ============================================================================ 
// 1. 位置: 位于项目根目录下的 tests/ 文件夹。
// 2. 机制: 每个文件都被编译成一个独立的 Crate。
// 3. 访问权限: 就像外部用户一样，只能访问 `pub` 公开 API。

use cargo_mastery; // 必须像第三方库一样引入

#[test]
fn test_from_outside() {
    // 只能调用 pub 函数
    cargo_mastery::kitchen::order_breakfast();
    
    // 尝试调用私有函数会报错:
    // cargo_mastery::internal_adder(2, 2); // ❌ Error: function is private
}
