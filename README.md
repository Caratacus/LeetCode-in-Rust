# LeetCode-in-Rust

A Rust-based LeetCode algorithm practice project that lets anyone enjoy the feel of LeetCode Premium.

Ported from [LeetCode-in-Java](https://github.com/javadev/LeetCode-in-Java).

## Project Structure

```
src/
├── lib.rs                    # Root module declarations
├── common/                   # Shared data structures
│   ├── list_node.rs          # Singly-linked list node
│   ├── tree_node.rs          # Binary tree node
│   ├── graph_node.rs         # Graph node (clone graph)
│   ├── random_node.rs        # Linked list node with random pointer
│   ├── left_right_node.rs    # Tree node with next pointer
│   ├── employee.rs           # Employee struct
│   └── nested_integer.rs     # Nested integer enum
├── utils/                    # Test utility functions
│   ├── linked_list_utils.rs  # linked_list_from_vec / linked_list_to_vec
│   ├── tree_utils.rs         # tree_from_vec
│   └── array_utils.rs        # compare_2d_unsorted
├── s0001/
│   └── two_sum/
│       ├── mod.rs            # Solution struct + impl + tests
│       ├── readme.md         # Problem description
│       └── complexity.md     # Complexity analysis
├── s0002/
│   └── add_two_numbers/
│       ├── mod.rs
│       ├── readme.md
│       └── complexity.md
└── ...     
```

## Build & Test

```bash
cargo build          # Compile all 2,837 problem stubs
cargo test           # Run all tests (placeholder)
cargo check          # Quick check for errors
```

## Stats

- **2,837** pure Java algorithm problems ported
- **Excluded**: 100 SQL problems, 16 Pandas problems
- **Coverage**: Problem #1 through #3734
- **Edition**: Rust 2024

## License

MIT
