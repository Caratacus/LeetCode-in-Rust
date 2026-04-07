// Tests for Problem 1962: Remove Stones to Minimize the Total
// Java reference: src/test/java/g1901_2000/s1962_remove_stones_to_minimize_the_total/SolutionTest.java

use leetcode_in_rust::s1962::remove_stones_to_minimize_the_total::Solution;

#[test]
fn test_min_stone_sum() {
    assert_eq!(Solution::min_stone_sum(vec![5, 4, 9], 2), 12);
}

#[test]
fn test_min_stone_sum2() {
    assert_eq!(Solution::min_stone_sum(vec![4, 3, 6, 7], 3), 12);
}
