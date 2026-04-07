// Tests for Problem 2612: Minimum Reverse Operations
// Java reference: src/test/java/g2601_2700/s2612_minimum_reverse_operations/SolutionTest.java

use leetcode_in_rust::s2612::minimum_reverse_operations::Solution;

#[test]
fn test_min_reverse_operations() {
    assert_eq!(
        Solution::min_reverse_operations(4, 0, vec![1, 2], 4),
        vec![0, -1, -1, 1]
    );
}

#[test]
fn test_min_reverse_operations2() {
    assert_eq!(
        Solution::min_reverse_operations(5, 0, vec![2, 4], 3),
        vec![0, -1, -1, -1, -1]
    );
}
