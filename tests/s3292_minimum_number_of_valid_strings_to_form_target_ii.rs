// Tests for Problem 3292: Minimum Number of Valid Strings to Form Target II
// Java reference: src/test/java/g3201_3300/s3292_minimum_number_of_valid_strings_to_form_target_ii/SolutionTest.java

use leetcode_in_rust::s3292::minimum_number_of_valid_strings_to_form_target_ii::Solution;

#[test]
fn test_min_valid_strings() {
    assert_eq!(
        Solution::min_valid_strings(
            vec!["abc".to_string(), "aaaaa".to_string(), "bcdef".to_string()],
            "aabcdabc".to_string()
        ),
        3
    );
}

#[test]
fn test_min_valid_strings2() {
    assert_eq!(
        Solution::min_valid_strings(
            vec!["abababab".to_string(), "ab".to_string()],
            "ababaababa".to_string()
        ),
        2
    );
}

#[test]
fn test_min_valid_strings3() {
    assert_eq!(
        Solution::min_valid_strings(vec!["abcdef".to_string()], "xyz".to_string()),
        -1
    );
}
