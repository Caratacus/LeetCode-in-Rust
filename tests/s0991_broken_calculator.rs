// Tests for Problem 0991: Broken Calculator
// Java reference: src/test/java/g0901_1000/s0991_broken_calculator/SolutionTest.java

use leetcode_in_rust::s0991::broken_calculator::Solution;

#[test]
fn test_broken_calc() {
    assert_eq!(Solution::broken_calc(2, 3), 2);
}

#[test]
fn test_broken_calc2() {
    assert_eq!(Solution::broken_calc(5, 8), 2);
}

#[test]
fn test_broken_calc3() {
    assert_eq!(Solution::broken_calc(3, 10), 3);
}
