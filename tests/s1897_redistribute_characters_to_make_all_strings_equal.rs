// Tests for Problem 1897: Redistribute Characters to Make All Strings Equal
// Java reference: src/test/java/g1801_1900/s1897_redistribute_characters_to_make_all_strings_equal/SolutionTest.java

use leetcode_in_rust::s1897::redistribute_characters_to_make_all_strings_equal::Solution;

#[test]
fn test_make_equal() {
    assert_eq!(
        Solution::make_equal(vec!["abc".to_string(), "aabc".to_string(), "bc".to_string()]),
        true
    );
}

#[test]
fn test_make_equal2() {
    assert_eq!(
        Solution::make_equal(vec!["ab".to_string(), "a".to_string()]),
        false
    );
}
