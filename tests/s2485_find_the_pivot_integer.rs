// Tests for Problem 2485: Find the Pivot Integer
// Java reference: src/test/java/g2401_2500/s2485_find_the_pivot_integer/SolutionTest.java

use leetcode_in_rust::s2485::find_the_pivot_integer::Solution;

#[test]
fn test_pivot_integer() {
    assert_eq!(Solution::pivot_integer(8), 6);
}

#[test]
fn test_pivot_integer2() {
    assert_eq!(Solution::pivot_integer(1), 1);
}

#[test]
fn test_pivot_integer3() {
    assert_eq!(Solution::pivot_integer(4), -1);
}
