// Tests for Problem 1006: Clumsy Factorial
// Java reference: src/test/java/g1001_1100/s1006_clumsy_factorial/SolutionTest.java

use leetcode_in_rust::s1006::clumsy_factorial::Solution;

#[test]
fn test_clumsy() {
    assert_eq!(Solution::clumsy(4), 7);
}

#[test]
fn test_clumsy2() {
    assert_eq!(Solution::clumsy(10), 12);
}
