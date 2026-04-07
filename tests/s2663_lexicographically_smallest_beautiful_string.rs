// Tests for Problem 2663: Lexicographically Smallest Beautiful String
// Java reference: src/test/java/g2601_2700/s2663_lexicographically_smallest_beautiful_string/SolutionTest.java

use leetcode_in_rust::s2663::lexicographically_smallest_beautiful_string::Solution;

#[test]
fn test_smallest_beautiful_string() {
    assert_eq!(
        Solution::smallest_beautiful_string("abcz".to_string(), 26),
        "abda"
    );
}

#[test]
fn test_smallest_beautiful_string2() {
    assert_eq!(
        Solution::smallest_beautiful_string("dc".to_string(), 4),
        ""
    );
}
