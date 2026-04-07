// Tests for Problem 2024: Maximize the Confusion of an Exam
// Java reference: src/test/java/g2001_2100/s2024_maximize_the_confusion_of_an_exam/SolutionTest.java

use leetcode_in_rust::s2024::maximize_the_confusion_of_an_exam::Solution;

#[test]
fn test_max_consecutive_answers() {
    assert_eq!(Solution::max_consecutive_answers("TTFF".to_string(), 2), 4);
}

#[test]
fn test_max_consecutive_answers2() {
    assert_eq!(Solution::max_consecutive_answers("TTFF".to_string(), 1), 3);
}

#[test]
fn test_max_consecutive_answers3() {
    assert_eq!(Solution::max_consecutive_answers("TTFTTFTT".to_string(), 1), 5);
}
