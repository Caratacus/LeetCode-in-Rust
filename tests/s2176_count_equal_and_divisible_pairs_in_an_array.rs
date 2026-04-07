// Tests for Problem 2176: Count Equal and Divisible Pairs in an Array
// Java reference: src/test/java/g2101_2200/s2176_count_equal_and_divisible_pairs_in_an_array/SolutionTest.java

use leetcode_in_rust::s2176::count_equal_and_divisible_pairs_in_an_array::Solution;

#[test]
fn test_count_pairs() {
    assert_eq!(Solution::count_pairs(vec![3, 1, 2, 2, 2, 1, 3], 2), 4);
}

#[test]
fn test_count_pairs2() {
    assert_eq!(Solution::count_pairs(vec![1, 2, 3, 4], 1), 0);
}
