// Tests for Problem 3655: XOR After Range Multiplication Queries II
// Java reference: src/test/java/g3601_3700/s3655_xor_after_range_multiplication_queries_ii/SolutionTest.java
use leetcode_in_rust::s3655::xor_after_range_multiplication_queries_ii::Solution;
#[test]
fn test_xor_after_queries() {
    assert_eq!(Solution::xor_after_queries(vec![1, 1, 1], vec![vec![0, 2, 1, 4]]), 4);
}
#[test]
fn test_xor_after_queries2() {
    assert_eq!(Solution::xor_after_queries(vec![2, 3, 1, 5, 4], vec![vec![1, 4, 2, 3], vec![0, 2, 1, 2]]), 31);
}
#[test]
fn test_xor_after_queries3() {
    assert_eq!(Solution::xor_after_queries(vec![329, 112, 80], vec![vec![2, 2, 2, 20], vec![0, 2, 1, 19], vec![0, 2, 3, 9], vec![1, 2, 1, 11], vec![2, 2, 1, 11], vec![0, 2, 2, 11], vec![1, 1, 2, 2], vec![0, 1, 1, 14], vec![1, 2, 3, 8], vec![2, 2, 1, 14], vec![2, 2, 3, 10], vec![2, 2, 3, 1], vec![1, 1, 2, 12], vec![0, 2, 1, 15], vec![0, 2, 1, 3], vec![1, 1, 3, 15], vec![1, 1, 2, 2]]), 426005772);
}
