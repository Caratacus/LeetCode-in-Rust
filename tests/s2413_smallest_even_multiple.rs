// Tests for Problem 2413: Smallest Even Multiple
// Java reference: src/test/java/g2401_2500/s2413_smallest_even_multiple/SolutionTest.java

use leetcode_in_rust::s2413::smallest_even_multiple::Solution;

#[test]
fn test_smallest_even_multiple() {
    assert_eq!(Solution::smallest_even_multiple(5), 10);
}

#[test]
fn test_smallest_even_multiple2() {
    assert_eq!(Solution::smallest_even_multiple(6), 6);
}
