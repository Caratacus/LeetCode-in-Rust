// Tests for Problem 3473: Sum of K Subarrays with Length at Least M
// Java reference: src/test/java/g3401_3500/s3473_sum_of_k_subarrays_with_length_at_least_m/SolutionTest.java

use leetcode_in_rust::s3473::sum_of_k_subarrays_with_length_at_least_m::Solution;

#[test]
fn test_max_sum() {
    assert_eq!(Solution::max_sum(vec![1, 2, -1, 3, 3, 4], 2, 2), 13);
}

#[test]
fn test_max_sum2() {
    assert_eq!(Solution::max_sum(vec![-10, 3, -1, -2], 4, 1), -10);
}
