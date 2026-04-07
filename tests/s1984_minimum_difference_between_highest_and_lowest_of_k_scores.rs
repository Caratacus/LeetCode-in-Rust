// Tests for Problem 1984: Minimum Difference Between Highest and Lowest of K Scores
// Java reference: src/test/java/g1901_2000/s1984_minimum_difference_between_highest_and_lowest_of_k_scores/SolutionTest.java

use leetcode_in_rust::s1984::minimum_difference_between_highest_and_lowest_of_k_scores::Solution;

#[test]
fn test_minimum_difference() {
    assert_eq!(Solution::minimum_difference(vec![90], 1), 0);
}

#[test]
fn test_minimum_difference2() {
    assert_eq!(Solution::minimum_difference(vec![9, 4, 1, 7], 2), 2);
}
