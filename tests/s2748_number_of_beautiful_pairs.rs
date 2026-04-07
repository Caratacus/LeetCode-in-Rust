// Tests for Problem 2748: Number of Beautiful Pairs
// Java reference: src/test/java/g2701_2800/s2748_number_of_beautiful_pairs/SolutionTest.java

use leetcode_in_rust::s2748::number_of_beautiful_pairs::Solution;

#[test]
fn test_count_beautiful_pairs() {
    assert_eq!(Solution::count_beautiful_pairs(vec![2, 5, 1, 4]), 5);
}

#[test]
fn test_count_beautiful_pairs2() {
    assert_eq!(Solution::count_beautiful_pairs(vec![11, 21, 12]), 2);
}
