// Tests for Problem 2309: Greatest English Letter in Upper and Lower Case
// Java reference: src/test/java/g2301_2400/s2309_greatest_english_letter_in_upper_and_lower_case/SolutionTest.java

use leetcode_in_rust::s2309::greatest_english_letter_in_upper_and_lower_case::Solution;

#[test]
fn test_greatest_letter() {
    assert_eq!(
        Solution::greatest_letter(String::from("lEeTcOdE")),
        String::from("E")
    );
}

#[test]
fn test_greatest_letter2() {
    assert_eq!(
        Solution::greatest_letter(String::from("arRAzFif")),
        String::from("R")
    );
}

#[test]
fn test_greatest_letter3() {
    assert_eq!(Solution::greatest_letter(String::from("")), String::from(""));
}
