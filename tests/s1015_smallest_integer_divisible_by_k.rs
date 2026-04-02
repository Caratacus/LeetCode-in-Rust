// Tests for Problem 1015: Smallest Integer Divisible by K
// Java reference: src/test/java/g1001_1100/s1015_smallest_integer_divisible_by_k/SolutionTest.java

use leetcode_in_rust::s1015::smallest_integer_divisible_by_k::Solution;

#[test]
fn test_smallest_repunit_div_by_k() {
    assert_eq!(Solution::smallest_repunit_div_by_k(1), 1);
}

#[test]
fn test_smallest_repunit_div_by_k2() {
    assert_eq!(Solution::smallest_repunit_div_by_k(2), -1);
}

#[test]
fn test_smallest_repunit_div_by_k3() {
    assert_eq!(Solution::smallest_repunit_div_by_k(3), 3);
}
