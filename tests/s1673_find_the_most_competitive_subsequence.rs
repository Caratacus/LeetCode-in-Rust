// Tests for Problem 1673: Find the Most Competitive Subsequence
// Java reference: src/test/java/g1601_1700/s1673_find_the_most_competitive_subsequence/SolutionTest.java

use leetcode_in_rust::s1673::find_the_most_competitive_subsequence::Solution;

#[test]
fn test_most_competitive() {
    assert_eq!(Solution::most_competitive(vec![3, 5, 2, 6], 2), vec![2, 6]);
}

#[test]
fn test_most_competitive2() {
    assert_eq!(Solution::most_competitive(vec![2, 4, 3, 3, 5, 4, 9, 6], 4), vec![2, 3, 3, 4]);
}
