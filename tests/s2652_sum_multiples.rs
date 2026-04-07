// Tests for Problem 2652: Sum Multiples
// Java reference: src/test/java/g2601_2700/s2652_sum_multiples/SolutionTest.java

use leetcode_in_rust::s2652::sum_multiples::Solution;

#[test]
fn test_sum_of_multiples() {
    assert_eq!(Solution::sum_of_multiples(7), 21);
}

#[test]
fn test_sum_of_multiples2() {
    assert_eq!(Solution::sum_of_multiples(10), 40);
}

#[test]
fn test_sum_of_multiples3() {
    assert_eq!(Solution::sum_of_multiples(9), 30);
}
