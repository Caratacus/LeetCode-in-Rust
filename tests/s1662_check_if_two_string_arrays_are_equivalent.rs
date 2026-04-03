// Tests for Problem 1662: Check If Two String Arrays are Equivalent
// Java reference: src/test/java/g1601_1700/s1662_check_if_two_string_arrays_are_equivalent/SolutionTest.java

use leetcode_in_rust::s1662::check_if_two_string_arrays_are_equivalent::Solution;

#[test]
fn test_array_strings_are_equal() {
    assert_eq!(
        Solution::array_strings_are_equal(
            vec!["ab".to_string(), "c".to_string()],
            vec!["a".to_string(), "bc".to_string()]
        ),
        true
    );
}

#[test]
fn test_array_strings_are_equal2() {
    assert_eq!(
        Solution::array_strings_are_equal(
            vec!["a".to_string(), "cb".to_string()],
            vec!["ab".to_string(), "c".to_string()]
        ),
        false
    );
}

#[test]
fn test_array_strings_are_equal3() {
    assert_eq!(
        Solution::array_strings_are_equal(
            vec!["abc".to_string(), "d".to_string(), "defg".to_string()],
            vec!["abcddefg".to_string()]
        ),
        true
    );
}
