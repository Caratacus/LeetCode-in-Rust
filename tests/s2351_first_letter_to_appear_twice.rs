// Tests for Problem 2351: First Letter to Appear Twice
// Java reference: src/test/java/g2301_2400/s2351_first_letter_to_appear_twice/SolutionTest.java

use leetcode_in_rust::s2351::first_letter_to_appear_twice::Solution;

#[test]
fn test_repeated_character() {
    assert_eq!(Solution::repeated_character("abccbaacz".to_string()), 'c');
}

#[test]
fn test_repeated_character2() {
    assert_eq!(Solution::repeated_character("abcdd".to_string()), 'd');
}

#[test]
fn test_repeated_character3() {
    // Note: Java test expects '0' when no character appears twice
    // This may be a special case - actual implementation may differ
    assert_eq!(Solution::repeated_character("abcd".to_string()), '0');
}
