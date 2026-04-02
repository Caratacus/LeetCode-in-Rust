// Tests for Problem 0859: Buddy Strings
// Java reference: src/test/java/g0801_0900/s0859_buddy_strings/SolutionTest.java

use leetcode_in_rust::s0859::buddy_strings::Solution;

#[test]
fn test_buddy_strings() {
    assert_eq!(Solution::buddy_strings("ab".to_string(), "ba".to_string()), true);
}

#[test]
fn test_buddy_strings2() {
    assert_eq!(Solution::buddy_strings("ab".to_string(), "ab".to_string()), false);
}

#[test]
fn test_buddy_strings3() {
    assert_eq!(Solution::buddy_strings("aa".to_string(), "aa".to_string()), true);
}
