// Tests for Problem 3022: Minimize OR of Remaining Elements Using Operations
// Java reference: src/test/java/g3001_3100/s3022_minimize_or_of_remaining_elements_using_operations/SolutionTest.java
use leetcode_in_rust::s3022::minimize_or_of_remaining_elements_using_operations::Solution;
#[test]
fn test_min_or_after_operations() {
    assert_eq!(Solution::min_or_after_operations(vec![3, 5, 3, 2, 7], 2), 3);
}
#[test]
fn test_min_or_after_operations2() {
    assert_eq!(Solution::min_or_after_operations(vec![7, 3, 15, 14, 2, 8], 4), 2);
}
#[test]
fn test_min_or_after_operations3() {
    assert_eq!(Solution::min_or_after_operations(vec![10, 7, 10, 3, 9, 14, 9, 4], 1), 15);
}
