// Tests for Problem 1753: Maximum Score From Removing Stones
// Java reference: src/test/java/g1701_1800/s1753_maximum_score_from_removing_stones/SolutionTest.java

use leetcode_in_rust::s1753::maximum_score_from_removing_stones::Solution;

#[test]
fn test_maximum_score() {
    assert_eq!(Solution::maximum_score(2, 4, 6), 6);
}

#[test]
fn test_maximum_score2() {
    assert_eq!(Solution::maximum_score(4, 4, 6), 7);
}

#[test]
fn test_maximum_score3() {
    assert_eq!(Solution::maximum_score(1, 8, 8), 8);
}
