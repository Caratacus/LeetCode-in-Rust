// Tests for Problem 1952: Three Divisors
// Java reference: src/test/java/g1901_2000/s1952_three_divisors/SolutionTest.java

use leetcode_in_rust::s1952::three_divisors::Solution;

#[test]
fn test_is_three() {
    assert_eq!(Solution::is_three(2), false);
}

#[test]
fn test_is_three2() {
    assert_eq!(Solution::is_three(4), true);
}
