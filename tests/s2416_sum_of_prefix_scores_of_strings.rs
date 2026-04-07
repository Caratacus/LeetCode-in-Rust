// Tests for Problem 2416: Sum of Prefix Scores of Strings
// Java reference: src/test/java/g2401_2500/s2416_sum_of_prefix_scores_of_strings/SolutionTest.java

use leetcode_in_rust::s2416::sum_of_prefix_scores_of_strings::Solution;

#[test]
fn test_sum_prefix_scores() {
    assert_eq!(
        Solution::sum_prefix_scores(vec![String::from("abc"), String::from("ab"), String::from("bc"), String::from("b")]),
        vec![5, 4, 3, 2]
    );
}

#[test]
fn test_sum_prefix_scores2() {
    assert_eq!(
        Solution::sum_prefix_scores(vec![String::from("abcd")]),
        vec![4]
    );
}
