// Tests for Problem 1910: Remove All Occurrences of a Substring
// Java reference: src/test/java/g1901_2000/s1910_remove_all_occurrences_of_a_substring/SolutionTest.java

use leetcode_in_rust::s1910::remove_all_occurrences_of_a_substring::Solution;

#[test]
fn test_remove_occurrences() {
    assert_eq!(
        Solution::remove_occurrences("daabcbaabcbc".to_string(), "abc".to_string()),
        "dab".to_string()
    );
}

#[test]
fn test_remove_occurrences2() {
    assert_eq!(
        Solution::remove_occurrences("axxxxyyyyb".to_string(), "xy".to_string()),
        "ab".to_string()
    );
}
