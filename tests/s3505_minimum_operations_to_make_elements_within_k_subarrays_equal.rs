// Tests for Problem 3505: Minimum Operations to Make Elements Within K Subarrays Equal
// Java reference: src/test/java/g3501_3600/s3505_minimum_operations_to_make_elements_within_k_subarrays_equal/SolutionTest.java

use leetcode_in_rust::s3505::minimum_operations_to_make_elements_within_k_subarrays_equal::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(
        Solution::min_operations(vec![5, -2, 1, 3, 7, 3, 6, 4, -1], 3, 2),
        8i64
    );
}

#[test]
fn test_min_operations2() {
    assert_eq!(
        Solution::min_operations(vec![9, -2, -2, -2, 1, 5], 2, 2),
        3i64
    );
}
