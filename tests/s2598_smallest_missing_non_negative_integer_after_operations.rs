// Tests for Problem 2598: Smallest Missing Non-negative Integer After Operations
// Java reference: src/test/java/g2501_2600/s2598_smallest_missing_non_negative_integer_after_operations/SolutionTest.java

use leetcode_in_rust::s2598::smallest_missing_non_negative_integer_after_operations::Solution;

#[test]
fn test_find_smallest_integer() {
    assert_eq!(
        Solution::find_smallest_integer(vec![1, -10, 7, 13, 6, 8], 5),
        4
    );
}

#[test]
fn test_find_smallest_integer2() {
    assert_eq!(
        Solution::find_smallest_integer(vec![1, -10, 7, 13, 6, 8], 7),
        2
    );
}
