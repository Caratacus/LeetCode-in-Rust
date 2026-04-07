// Tests for Problem 2564: Substring XOR Queries
// Java reference: src/test/java/g2501_2600/s2564_substring_xor_queries/SolutionTest.java

use leetcode_in_rust::s2564::substring_xor_queries::Solution;

#[test]
fn test_substring_xor_queries() {
    assert_eq!(
        Solution::substring_xor_queries("101101".to_string(), vec![vec![0, 5], vec![1, 2]]),
        vec![vec![0, 2], vec![2, 3]]
    );
}

#[test]
fn test_substring_xor_queries2() {
    assert_eq!(
        Solution::substring_xor_queries("0101".to_string(), vec![vec![12, 8]]),
        vec![vec![-1, -1]]
    );
}

#[test]
fn test_substring_xor_queries3() {
    assert_eq!(
        Solution::substring_xor_queries("1".to_string(), vec![vec![4, 5]]),
        vec![vec![0, 0]]
    );
}
