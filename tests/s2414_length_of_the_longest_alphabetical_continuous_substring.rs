// Tests for Problem 2414: Length of the Longest Alphabetical Continuous Substring
// Java reference: src/test/java/g2401_2500/s2414_length_of_the_longest_alphabetical_continuous_substring/SolutionTest.java

use leetcode_in_rust::s2414::length_of_the_longest_alphabetical_continuous_substring::Solution;

#[test]
fn test_longest_continuous_substring() {
    assert_eq!(Solution::longest_continuous_substring(String::from("abacaba")), 2);
}

#[test]
fn test_longest_continuous_substring2() {
    assert_eq!(Solution::longest_continuous_substring(String::from("abcde")), 5);
}
