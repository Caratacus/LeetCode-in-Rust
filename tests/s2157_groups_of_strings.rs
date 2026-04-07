// Tests for Problem 2157: Groups of Strings
// Java reference: src/test/java/g2101_2200/s2157_groups_of_strings/SolutionTest.java

use leetcode_in_rust::s2157::groups_of_strings::Solution;

#[test]
fn test_group_strings() {
    assert_eq!(
        Solution::group_strings(vec!["a".to_string(), "b".to_string(), "ab".to_string(), "cde".to_string()]),
        vec![2, 3]
    );
}

#[test]
fn test_group_strings2() {
    assert_eq!(
        Solution::group_strings(vec!["a".to_string(), "ab".to_string(), "abc".to_string()]),
        vec![1, 3]
    );
}
