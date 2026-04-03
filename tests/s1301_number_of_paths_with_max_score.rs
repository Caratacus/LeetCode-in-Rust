// Tests for Problem 1301: Number of Paths with Max Score
// Java reference: src/test/java/g1301_1400/s1301_number_of_paths_with_max_score/SolutionTest.java

use leetcode_in_rust::s1301::number_of_paths_with_max_score::Solution;

#[test]
fn test_paths_with_max_score() {
    assert_eq!(
        Solution::paths_with_max_score(vec!["E23".to_string(), "2X2".to_string(), "12S".to_string()]),
        vec![7, 1]
    );
}

#[test]
fn test_paths_with_max_score2() {
    assert_eq!(
        Solution::paths_with_max_score(vec!["E12".to_string(), "1X1".to_string(), "21S".to_string()]),
        vec![4, 2]
    );
}

#[test]
fn test_paths_with_max_score3() {
    assert_eq!(
        Solution::paths_with_max_score(vec!["E11".to_string(), "XXX".to_string(), "11S".to_string()]),
        vec![0, 0]
    );
}
