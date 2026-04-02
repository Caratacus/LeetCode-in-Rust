// Tests for Problem 0784: Letter Case Permutation
// Java reference: src/test/java/g0701_0800/s0784_letter_case_permutation/SolutionTest.java

use leetcode_in_rust::s0784::letter_case_permutation::Solution;

#[test]
fn test_letter_case_permutation() {
    let mut result = Solution::letter_case_permutation("a1b2".to_string());
    result.sort();
    let mut expected = vec!["a1b2".to_string(), "a1B2".to_string(), "A1b2".to_string(), "A1B2".to_string()];
    expected.sort();
    assert_eq!(result, expected);
}

#[test]
fn test_letter_case_permutation2() {
    let mut result = Solution::letter_case_permutation("3z4".to_string());
    result.sort();
    let mut expected = vec!["3z4".to_string(), "3Z4".to_string()];
    expected.sort();
    assert_eq!(result, expected);
}

#[test]
fn test_letter_case_permutation3() {
    let mut result = Solution::letter_case_permutation("C".to_string());
    result.sort();
    let mut expected = vec!["C".to_string(), "c".to_string()];
    expected.sort();
    assert_eq!(result, expected);
}
