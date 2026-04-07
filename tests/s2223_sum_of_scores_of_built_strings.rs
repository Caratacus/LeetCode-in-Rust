// Tests for Problem 2223: Sum of Scores of Built Strings
// Java reference: src/test/java/g2201_2300/s2223_sum_of_scores_of_built_strings/SolutionTest.java

use leetcode_in_rust::s2223::sum_of_scores_of_built_strings::Solution;

#[test]
fn test_sum_scores() {
    assert_eq!(Solution::sum_scores("babab".to_string()), 9);
}

#[test]
fn test_sum_scores2() {
    assert_eq!(Solution::sum_scores("azbazbzaz".to_string()), 14);
}
