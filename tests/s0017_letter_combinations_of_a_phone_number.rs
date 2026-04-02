// Tests for Problem 0017: Letter Combinations of a Phone Number
// Java reference: src/test/java/g0001_0100/s0017_letter_combinations_of_a_phone_number/SolutionTest.java

use leetcode_in_rust::s0017::letter_combinations_of_a_phone_number::Solution;

#[test]
fn test_letter_combinations() {
    let result = Solution::letter_combinations("23".to_string());
    let expected = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];
    assert_eq!(result, expected);
}

#[test]
fn test_letter_combinations2() {
    let result = Solution::letter_combinations("".to_string());
    let expected: Vec<String> = vec![];
    assert_eq!(result, expected);
}

#[test]
fn test_letter_combinations3() {
    let result = Solution::letter_combinations("2".to_string());
    let expected = vec!["a", "b", "c"];
    assert_eq!(result, expected);
}
