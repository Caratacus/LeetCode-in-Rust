// Tests for Problem 0095: Unique Binary Search Trees II
// Java reference: src/test/java/g0001_0100/s0095_unique_binary_search_trees_ii/SolutionTest.java

use leetcode_in_rust::s0095::unique_binary_search_trees_ii::Solution;

#[test]
fn test_generate_trees() {
    let result = Solution::generate_trees(3);
    // Should generate 5 different BSTs
    assert_eq!(result.len(), 5);
}

#[test]
fn test_generate_trees2() {
    let result = Solution::generate_trees(1);
    assert_eq!(result.len(), 1);
}
