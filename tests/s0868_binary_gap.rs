// Tests for Problem 0868: Binary Gap
// Java reference: src/test/java/g0801_0900/s0868_binary_gap/SolutionTest.java

use leetcode_in_rust::s0868::binary_gap::Solution;

#[test]
fn test_binary_gap() {
    assert_eq!(Solution::binary_gap(22), 2);
}

#[test]
fn test_binary_gap2() {
    assert_eq!(Solution::binary_gap(8), 0);
}

#[test]
fn test_binary_gap3() {
    assert_eq!(Solution::binary_gap(5), 2);
}
