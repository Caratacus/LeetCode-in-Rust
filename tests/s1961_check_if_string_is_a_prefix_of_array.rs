// Tests for Problem 1961: Check If String Is a Prefix of Array
// Java reference: src/test/java/g1901_2000/s1961_check_if_string_is_a_prefix_of_array/SolutionTest.java

use leetcode_in_rust::s1961::check_if_string_is_a_prefix_of_array::Solution;

#[test]
fn test_is_prefix_string() {
    assert_eq!(
        Solution::is_prefix_string(
            "iloveleetcode".to_string(),
            vec!["i".to_string(), "love".to_string(), "leetcode".to_string(), "apples".to_string()]
        ),
        true
    );
}

#[test]
fn test_is_prefix_string2() {
    assert_eq!(
        Solution::is_prefix_string(
            "iloveleetcode".to_string(),
            vec!["apples".to_string(), "i".to_string(), "love".to_string(), "leetcode".to_string()]
        ),
        false
    );
}
