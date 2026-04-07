// Tests for Problem 2302: Count Subarrays With Score Less Than K
// Java reference: src/test/java/g2301_2400/s2302_count_subarrays_with_score_less_than_k/SolutionTest.java

use leetcode_in_rust::s2302::count_subarrays_with_score_less_than_k::Solution;

#[test]
fn test_count_subarrays() {
    assert_eq!(Solution::count_subarrays(vec![2, 1, 4, 3, 5], 10), 6);
}

#[test]
fn test_count_subarrays2() {
    assert_eq!(Solution::count_subarrays(vec![1, 1, 1], 5), 5);
}
