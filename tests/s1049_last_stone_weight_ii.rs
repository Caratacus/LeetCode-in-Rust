// Tests for Problem 1049: Last Stone Weight II
// Java reference: src/test/java/g1001_1100/s1049_last_stone_weight_ii/SolutionTest.java

use leetcode_in_rust::s1049::last_stone_weight_ii::Solution;

#[test]
fn test_last_stone_weight_ii() {
    assert_eq!(Solution::last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1]), 1);
}

#[test]
fn test_last_stone_weight_ii2() {
    assert_eq!(Solution::last_stone_weight_ii(vec![31, 26, 33, 21, 40]), 5);
}
