# LeetCode in Rust - 项目规范

## 项目结构

```
leetcode-in-rust/
├── src/                           # 源代码目录
│   ├── common/                    # 公共数据结构
│   │   ├── list_node.rs           # 链表节点
│   │   ├── tree_node.rs           # 树节点
│   │   └── ...
│   ├── utils/                     # 工具函数
│   │   ├── linked_list_utils.rs   # 链表工具
│   │   ├── tree_utils.rs          # 树工具
│   │   └── ...
│   ├── s0001/                     # 题目按编号组织
│   │   └── two_sum/
│   │       └── mod.rs
│   ├── s0002/
│   │   └── add_two_numbers/
│   │       └── mod.rs
│   └── lib.rs                     # 库入口
│
└── tests/                         # 集成测试目录（与 Java 项目结构对应）
    ├── s0001_two_sum.rs
    ├── s0002_add_two_numbers.rs
    └── ...
```

## 命名规范

### 源代码文件

- **目录命名**: `s{题号}`，如 `s0001`, `s0002`
- **子目录命名**: 使用 snake_case 的题目名称，如 `two_sum`, `add_two_numbers`
- **文件命名**: 统一使用 `mod.rs`

### 测试文件

- **文件命名**: `s{题号}_{题目名称}.rs`，如 `s0001_two_sum.rs`
- **位置**: 放在 `tests/` 目录下，每个题目一个独立文件

### 测试函数

- **命名**: `test_{方法名}[序号]`，序号从 2 开始（无序号为第一个）
- **示例**:
  ```rust
  #[test]
  fn test_two_sum() { ... }      // 第一个测试

  #[test]
  fn test_two_sum2() { ... }     // 第二个测试

  #[test]
  fn test_two_sum3() { ... }     // 第三个测试
  ```

## 测试规范

### 测试文件模板

```rust
// Tests for Problem {题号}: {题目名称}
// Java reference: src/test/java/g{题号范围}/s{题号}_{题目名称}/SolutionTest.java

use leetcode_in_rust::s{题号}::{题目名称}::Solution;
// 如需工具函数:
// use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_{方法名}() {
    // 测试代码
}

#[test]
fn test_{方法名}2() {
    // 测试代码
}
```

### 测试用例要求

1. **与 Java 项目保持一致**: 测试用例必须与 `LeetCode-in-Java` 项目中的测试用例完全对应
2. **每个方法至少一个测试**: 确保 `Solution` 中的每个公开方法都有测试覆盖
3. **边界条件**: 包含空输入、单元素、边界值等测试用例

### 运行测试

```bash
# 运行单个题目的测试
cargo test --test s0001_two_sum

# 运行所有题目测试
cargo test --test 's000*'

# 运行所有测试（包括单元测试和集成测试）
cargo test
```

## 代码规范

### Solution 结构

```rust
// Problem {题号}: {题目名称}
// #{难度} #{标签} #{复杂度}

pub struct Solution;

impl Solution {
    pub fn {方法名}({参数}) -> {返回类型} {
        // 实现
    }
}
```

### 复杂度注释格式

```
// #Big_O_Time_O(n)_Space_O(1)
// #Big_O_Time_O(max(N,M))_Space_O(max(N,M))
// #Big_O_Time_O(log(min(N,M)))_Space_O(1)
```

### 常用导入

```rust
// 链表相关
use crate::common::list_node::ListNode;
use crate::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

// 树相关
use crate::common::tree_node::TreeNode;
use crate::utils::tree_utils::{tree_from_vec, tree_to_vec};

// HashMap
use std::collections::HashMap;
```

## 与 Java 项目对应关系

| Java | Rust |
|------|------|
| `src/main/java/g0001_0100/s0001_two_sum/Solution.java` | `src/s0001/two_sum/mod.rs` |
| `src/test/java/g0001_0100/s0001_two_sum/SolutionTest.java` | `tests/s0001_two_sum.rs` |
| `com_github_leetcode.LinkedListUtils` | `crate::utils::linked_list_utils` |
| `com_github_leetcode.TreeNode` | `crate::common::tree_node::TreeNode` |

## 注意事项

1. **不在源文件中写测试**: 所有测试必须放在 `tests/` 目录下
2. **保持测试独立性**: 每个测试文件独立，可单独运行
3. **注释 Java 参考**: 每个测试文件开头注明对应的 Java 测试文件路径
