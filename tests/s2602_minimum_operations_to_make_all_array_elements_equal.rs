// Tests for Problem 2602: Minimum Operations to Make All Array Elements Equal
// Java reference: src/test/java/g2601_2700/s2602_minimum_operations_to_make_all_array_elements_equal/SolutionTest.java

use leetcode_in_rust::s2602::minimum_operations_to_make_all_array_elements_equal::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(
        Solution::min_operations(vec![3, 1, 6, 8], vec![1, 5]),
        vec![14, 10]
    );
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations(vec![2, 9, 6, 3], vec![10]), vec![20]);
}
