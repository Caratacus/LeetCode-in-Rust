// Tests for Problem 3438: Find Valid Pair of Adjacent Digits in String
// Java reference: src/test/java/g3401_3500/s3438_find_valid_pair_of_adjacent_digits_in_string/SolutionTest.java

use leetcode_in_rust::s3438::find_valid_pair_of_adjacent_digits_in_string::Solution;

#[test]
fn test_find_valid_pair() {
    assert_eq!(Solution::find_valid_pair("2523533".to_string()), "23".to_string());
}

#[test]
fn test_find_valid_pair2() {
    assert_eq!(Solution::find_valid_pair("221".to_string()), "21".to_string());
}

#[test]
fn test_find_valid_pair3() {
    assert_eq!(Solution::find_valid_pair("22".to_string()), "".to_string());
}
