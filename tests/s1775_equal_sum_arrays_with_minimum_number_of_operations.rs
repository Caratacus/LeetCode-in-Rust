// Tests for Problem 1775: Equal Sum Arrays With Minimum Number of Operations
// Java reference: src/test/java/g1701_1800/s1775_equal_sum_arrays_with_minimum_number_of_operations/SolutionTest.java

use leetcode_in_rust::s1775::equal_sum_arrays_with_minimum_number_of_operations::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(
        Solution::min_operations(vec![1, 2, 3, 4, 5, 6], vec![1, 1, 2, 2, 2, 2]),
        3
    );
}

#[test]
fn test_min_operations2() {
    assert_eq!(
        Solution::min_operations(vec![1, 1, 1, 1, 1, 1, 1], vec![6]),
        -1
    );
}

#[test]
fn test_min_operations3() {
    assert_eq!(Solution::min_operations(vec![6, 6], vec![1]), 3);
}
