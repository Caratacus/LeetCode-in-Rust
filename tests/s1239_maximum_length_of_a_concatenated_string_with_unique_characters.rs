// Tests for Problem 1239: Maximum Length of a Concatenated String with Unique Characters
// Java reference: src/test/java/g1201_1300/s1239_maximum_length_of_a_concatenated_string_with_unique_characters/SolutionTest.java

use leetcode_in_rust::s1239::maximum_length_of_a_concatenated_string_with_unique_characters::Solution;

#[test]
fn test_max_length() {
    assert_eq!(
        Solution::max_length(vec!["un".to_string(), "iq".to_string(), "ue".to_string()]),
        4
    );
}

#[test]
fn test_max_length2() {
    assert_eq!(
        Solution::max_length(vec![
            "cha".to_string(),
            "r".to_string(),
            "act".to_string(),
            "ers".to_string()
        ]),
        6
    );
}

#[test]
fn test_max_length3() {
    assert_eq!(
        Solution::max_length(vec!["abcdefghijklmnopqrstuvwxyz".to_string()]),
        26
    );
}
