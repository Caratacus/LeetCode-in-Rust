// Tests for Problem 2259: Remove Digit From Number to Maximize Result
// Java reference: src/test/java/g2201_2300/s2259_remove_digit_from_number_to_maximize_result/SolutionTest.java

use leetcode_in_rust::s2259::remove_digit_from_number_to_maximize_result::Solution;

#[test]
fn test_remove_digit() {
    assert_eq!(Solution::remove_digit("123".to_string(), '3'), "12");
}

#[test]
fn test_remove_digit2() {
    assert_eq!(Solution::remove_digit("1231".to_string(), '1'), "231");
}

#[test]
fn test_remove_digit3() {
    assert_eq!(Solution::remove_digit("551".to_string(), '5'), "51");
}
