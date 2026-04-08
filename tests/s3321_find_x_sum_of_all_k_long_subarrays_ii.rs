// Tests for Problem 3321: Find X-Sum of All K-Long Subarrays II
// Java reference: src/test/java/g3301_3400/s3321_find_x_sum_of_all_k_long_subarrays_ii/SolutionTest.java

use leetcode_in_rust::s3321::find_x_sum_of_all_k_long_subarrays_ii::Solution;

#[test]
fn test_find_x_sum() {
    assert_eq!(
        Solution::find_x_sum(vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2),
        vec![6, 10, 12]
    );
}

#[test]
fn test_find_x_sum2() {
    assert_eq!(
        Solution::find_x_sum(vec![3, 8, 7, 8, 7, 5], 2, 2),
        vec![11, 15, 15, 15, 12]
    );
}
