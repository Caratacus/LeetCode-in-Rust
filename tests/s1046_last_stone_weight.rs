// Tests for Problem 1046: Last Stone Weight
// Java reference: src/test/java/g1001_1100/s1046_last_stone_weight/SolutionTest.java

use leetcode_in_rust::s1046::last_stone_weight::Solution;

#[test]
fn test_last_stone_weight() {
    assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
}

#[test]
fn test_last_stone_weight2() {
    assert_eq!(Solution::last_stone_weight(vec![1]), 1);
}
