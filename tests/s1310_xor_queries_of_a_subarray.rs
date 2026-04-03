// Tests for Problem 1310: XOR Queries of a Subarray
// Java reference: src/test/java/g1301_1400/s1310_xor_queries_of_a_subarray/SolutionTest.java

use leetcode_in_rust::s1310::xor_queries_of_a_subarray::Solution;

#[test]
fn test_xor_queries() {
    assert_eq!(
        Solution::xor_queries(vec![1, 3, 4, 8], vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]]),
        vec![2, 7, 14, 8]
    );
}

#[test]
fn test_xor_queries2() {
    assert_eq!(
        Solution::xor_queries(vec![4, 8, 2, 10], vec![vec![2, 3], vec![1, 3], vec![0, 0], vec![0, 3]]),
        vec![8, 0, 4, 4]
    );
}
