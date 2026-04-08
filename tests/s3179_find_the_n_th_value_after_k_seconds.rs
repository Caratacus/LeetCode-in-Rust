// Tests for Problem 3179: Find the N-th Value After K Seconds
// Java reference: src/test/java/g3101_3200/s3179_find_the_n_th_value_after_k_seconds/SolutionTest.java

use leetcode_in_rust::s3179::find_the_n_th_value_after_k_seconds::Solution;
#[test]
fn test_value_after_k_seconds() {
    assert_eq!(Solution::value_after_k_seconds(4, 5), 56);
}
#[test]
fn test_value_after_k_seconds2() {
    assert_eq!(Solution::value_after_k_seconds(5, 3), 35);
}
