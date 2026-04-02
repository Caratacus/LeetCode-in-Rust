// Tests for Problem 1047: Remove All Adjacent Duplicates In String
// Java reference: src/test/java/g1001_1100/s1047_remove_all_adjacent_duplicates_in_string/SolutionTest.java

use leetcode_in_rust::s1047::remove_all_adjacent_duplicates_in_string::Solution;

#[test]
fn test_remove_duplicates() {
    assert_eq!(Solution::remove_duplicates("abbaca".to_string()), "ca");
}

#[test]
fn test_remove_duplicates2() {
    assert_eq!(Solution::remove_duplicates("azxxzy".to_string()), "ay");
}
