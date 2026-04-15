// Tests for Problem 3685: Subsequence Sum After Capping Elements
// Java reference: src/test/java/g3601_3700/s3685_subsequence_sum_after_capping_elements/SolutionTest.java
use leetcode_in_rust::s3685::subsequence_sum_after_capping_elements::Solution;
#[test]
fn test_subsequence_sum_after_capping() {
    assert_eq!(Solution::subsequence_sum_after_capping(vec![4, 3, 2, 4], 5), vec![false, false, true, true]);
}
#[test]
fn test_subsequence_sum_after_capping2() {
    assert_eq!(Solution::subsequence_sum_after_capping(vec![1, 2, 3, 4, 5], 3), vec![true, true, true, true, true]);
}
