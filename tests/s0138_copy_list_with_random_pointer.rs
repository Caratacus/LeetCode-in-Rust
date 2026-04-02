// Tests for Problem 0138: Copy List with Random Pointer
// Java reference: src/test/java/g0121_0200/s0138_copy_list_with_random_pointer/SolutionTest.java

use leetcode_in_rust::s0138::copy_list_with_random_pointer::Solution;

#[test]
fn test_copy_random_list() {
    // Test with null
    let result = Solution::copy_random_list(None);
    assert!(result.is_none());
}
