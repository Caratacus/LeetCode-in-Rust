// Tests for Problem 1688: Count of Matches in Tournament
// Java reference: src/test/java/g1601_1700/s1688_count_of_matches_in_tournament/SolutionTest.java

use leetcode_in_rust::s1688::count_of_matches_in_tournament::Solution;

#[test]
fn test_number_of_matches() {
    assert_eq!(Solution::number_of_matches(7), 6);
}

#[test]
fn test_number_of_matches2() {
    assert_eq!(Solution::number_of_matches(14), 13);
}
