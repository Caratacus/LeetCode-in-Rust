// Tests for Problem 3365: Rearrange K Substrings to Form Target String
// Java reference: src/test/java/g3301_3400/s3365_rearrange_k_substrings_to_form_target_string/SolutionTest.java

use leetcode_in_rust::s3365::rearrange_k_substrings_to_form_target_string::Solution;

#[test]
fn test_is_possible_to_rearrange() {
    assert_eq!(
        Solution::is_possible_to_rearrange("abcd".to_string(), "cdab".to_string(), 2),
        true
    );
}

#[test]
fn test_is_possible_to_rearrange2() {
    assert_eq!(
        Solution::is_possible_to_rearrange("aabbcc".to_string(), "bbaacc".to_string(), 3),
        true
    );
}

#[test]
fn test_is_possible_to_rearrange3() {
    assert_eq!(
        Solution::is_possible_to_rearrange("aabbcc".to_string(), "bbaacc".to_string(), 2),
        false
    );
}
