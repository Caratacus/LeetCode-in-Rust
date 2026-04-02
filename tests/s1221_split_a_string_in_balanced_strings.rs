// Tests for Problem 1221: Split a String in Balanced Strings
// Java reference: src/test/java/g1201_1300/s1221_split_a_string_in_balanced_strings/SolutionTest.java

use leetcode_in_rust::s1221::split_a_string_in_balanced_strings::Solution;

#[test]
fn test_balanced_string_split() {
    assert_eq!(Solution::balanced_string_split("RLRRLLRLRL".to_string()), 4);
}

#[test]
fn test_balanced_string_split2() {
    assert_eq!(Solution::balanced_string_split("RLLLLRRRLR".to_string()), 3);
}

#[test]
fn test_balanced_string_split3() {
    assert_eq!(Solution::balanced_string_split("LLLLRRRR".to_string()), 1);
}
