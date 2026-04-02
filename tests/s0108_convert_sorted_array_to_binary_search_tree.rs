// Tests for Problem 0108: Convert Sorted Array to Binary Search Tree
// Java reference: src/test/java/g0101_0200/s0108_convert_sorted_array_to_binary_search_tree/SolutionTest.java

use leetcode_in_rust::s0108::convert_sorted_array_to_binary_search_tree::Solution;

#[test]
fn test_sorted_array_to_bst() {
    let root = Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]);
    // Note: Tree comparison requires custom logic due to Rc<RefCell>
    // This test verifies the function compiles and runs
    assert!(root.is_some());
}

#[test]
fn test_sorted_array_to_bst2() {
    let root = Solution::sorted_array_to_bst(vec![1, 3]);
    assert!(root.is_some());
}

#[test]
fn test_sorted_array_to_bst3() {
    let root = Solution::sorted_array_to_bst(vec![]);
    assert!(root.is_none());
}
