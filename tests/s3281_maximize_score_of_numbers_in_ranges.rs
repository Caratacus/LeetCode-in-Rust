// Tests for Problem 3281: Maximize Score of Numbers in Ranges
// Java reference: src/test/java/g3201_3300/s3281_maximize_score_of_numbers_in_ranges/SolutionTest.java

use leetcode_in_rust::s3281::maximize_score_of_numbers_in_ranges::Solution;

#[test]
fn test_max_possible_score() {
    assert_eq!(Solution::max_possible_score(vec![6, 0, 3], 2), 4);
}

#[test]
fn test_max_possible_score2() {
    assert_eq!(Solution::max_possible_score(vec![2, 6, 13, 13], 5), 5);
}
