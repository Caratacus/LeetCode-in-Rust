// Tests for Problem 1014: Best Sightseeing Pair
// Java reference: src/test/java/g1001_1100/s1014_best_sightseeing_pair/SolutionTest.java

use leetcode_in_rust::s1014::best_sightseeing_pair::Solution;

#[test]
fn test_max_score_sightseeing_pair() {
    assert_eq!(Solution::max_score_sightseeing_pair(vec![8, 1, 5, 2, 6]), 11);
}

#[test]
fn test_max_score_sightseeing_pair2() {
    assert_eq!(Solution::max_score_sightseeing_pair(vec![1, 2]), 2);
}
