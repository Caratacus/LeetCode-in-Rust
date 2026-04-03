// Tests for Problem 1318: Minimum Flips to Make a OR b Equal to c
// Java reference: src/test/java/g1301_1400/s1318_minimum_flips_to_make_a_or_b_equal_to_c/SolutionTest.java

use leetcode_in_rust::s1318::minimum_flips_to_make_a_or_b_equal_to_c::Solution;

#[test]
fn test_min_flips() {
    assert_eq!(Solution::min_flips(2, 6, 5), 3);
}

#[test]
fn test_min_flips2() {
    assert_eq!(Solution::min_flips(4, 2, 7), 1);
}

#[test]
fn test_min_flips3() {
    assert_eq!(Solution::min_flips(1, 2, 3), 0);
}
