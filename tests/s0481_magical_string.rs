// Tests for Problem 0481: Magical String
// Java reference: src/test/java/g0401_0500/s0481_magical_string/SolutionTest.java

use leetcode_in_rust::s0481::magical_string::Solution;

#[test]
fn test_magical_string() {
    assert_eq!(Solution::magical_string(6), 3);
}

#[test]
fn test_magical_string2() {
    assert_eq!(Solution::magical_string(1), 1);
}
