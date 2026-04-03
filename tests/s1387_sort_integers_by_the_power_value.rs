// Tests for Problem 1387: Sort Integers by the Power Value
// Java reference: src/test/java/g1301_1400/s1387_sort_integers_by_the_power_value/SolutionTest.java

use leetcode_in_rust::s1387::sort_integers_by_the_power_value::Solution;

#[test]
fn test_get_kth() {
    assert_eq!(Solution::get_kth(12, 15, 2), 13);
}

#[test]
fn test_get_kth2() {
    assert_eq!(Solution::get_kth(7, 11, 4), 7);
}
