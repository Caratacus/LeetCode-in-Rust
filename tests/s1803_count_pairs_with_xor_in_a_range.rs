// Tests for Problem 1803: Count Pairs With XOR in a Range
// Java reference: src/test/java/g1801_1900/s1803_count_pairs_with_xor_in_a_range/SolutionTest.java

use leetcode_in_rust::s1803::count_pairs_with_xor_in_a_range::Solution;

#[test]
fn test_count_pairs() {
    assert_eq!(Solution::count_pairs(vec![1, 4, 2, 7], 2, 6), 6);
}

#[test]
fn test_count_pairs2() {
    assert_eq!(Solution::count_pairs(vec![9, 8, 4, 2, 1], 5, 14), 8);
}
