// Tests for Problem 2587: Rearrange Array to Maximize Prefix Score
// Java reference: src/test/java/g2501_2600/s2587_rearrange_array_to_maximize_prefix_score/SolutionTest.java

use leetcode_in_rust::s2587::rearrange_array_to_maximize_prefix_score::Solution;

#[test]
fn test_max_score() {
    assert_eq!(Solution::max_score(vec![2, -1, 0, 1, -3, 3, -3]), 6);
}

#[test]
fn test_max_score2() {
    assert_eq!(Solution::max_score(vec![-2, -3, 0]), 0);
}
