// Tests for Problem 2183: Count Array Pairs Divisible by K
// Java reference: src/test/java/g2101_2200/s2183_count_array_pairs_divisible_by_k/SolutionTest.java

use leetcode_in_rust::s2183::count_array_pairs_divisible_by_k::Solution;

#[test]
fn test_count_pairs() {
    assert_eq!(Solution::count_pairs(vec![1, 2, 3, 4, 5], 2), 7);
}

#[test]
fn test_count_pairs2() {
    assert_eq!(Solution::count_pairs(vec![1, 2, 3, 4], 5), 0);
}
