// Tests for Problem 3091: Apply Operations to Make Sum of Array Greater Than or Equal to K
// Java reference: src/test/java/g3001_3100/s3091_apply_operations_to_make_sum_of_array_greater_than_or_equal_to_k/SolutionTest.java

use leetcode_in_rust::s3091::apply_operations_to_make_sum_of_array_greater_than_or_equal_to_k::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations(11), 5);
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations(1), 0);
}
