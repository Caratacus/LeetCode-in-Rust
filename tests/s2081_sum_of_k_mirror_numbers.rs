// Tests for Problem 2081: Sum of K-Mirror Numbers
// Java reference: src/test/java/g2001_2100/s2081_sum_of_k_mirror_numbers/SolutionTest.java

use leetcode_in_rust::s2081::sum_of_k_mirror_numbers::Solution;

#[test]
fn test_k_mirror() {
    assert_eq!(Solution::k_mirror(2, 5), 25);
}

#[test]
fn test_k_mirror2() {
    assert_eq!(Solution::k_mirror(3, 7), 499);
}

#[test]
fn test_k_mirror3() {
    assert_eq!(Solution::k_mirror(7, 17), 203790);
}
