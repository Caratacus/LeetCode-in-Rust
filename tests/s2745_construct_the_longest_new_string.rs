// Tests for Problem 2745: Construct the Longest New String
// Java reference: src/test/java/g2701_2800/s2745_construct_the_longest_new_string/SolutionTest.java

use leetcode_in_rust::s2745::construct_the_longest_new_string::Solution;

#[test]
fn test_longest_string() {
    assert_eq!(Solution::longest_string(2, 5, 1), 12);
}

#[test]
fn test_longest_string2() {
    assert_eq!(Solution::longest_string(3, 2, 2), 14);
}
