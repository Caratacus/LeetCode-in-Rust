// Tests for Problem 3653: XOR After Range Multiplication Queries I
// Java reference: src/test/java/g3601_3700/s3653_xor_after_range_multiplication_queries_i/SolutionTest.java
use leetcode_in_rust::s3653::xor_after_range_multiplication_queries_i::Solution;
#[test]
fn test_xor_after_queries() {
    assert_eq!(Solution::xor_after_queries(vec![1, 1, 1], vec![vec![0, 2, 1, 4]]), 4);
}
#[test]
fn test_xor_after_queries2() {
    assert_eq!(Solution::xor_after_queries(vec![2, 3, 1, 5, 4], vec![vec![1, 4, 2, 3], vec![0, 2, 1, 2]]), 31);
}
