// Tests for Problem 0313: Super Ugly Number
// Java reference: src/test/java/g0301_0400/s0313_super_ugly_number/SolutionTest.java

use leetcode_in_rust::s0313::super_ugly_number::Solution;

#[test]
fn test_nth_super_ugly_number() {
    assert_eq!(Solution::nth_super_ugly_number(12, vec![2, 7, 13, 19]), 32);
}

#[test]
fn test_nth_super_ugly_number2() {
    assert_eq!(Solution::nth_super_ugly_number(1, vec![2, 3, 5]), 1);
}
