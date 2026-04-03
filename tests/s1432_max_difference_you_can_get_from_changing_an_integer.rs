// Tests for Problem 1432: Max Difference You Can Get from Changing an Integer
// Java reference: src/test/java/g1401_1500/s1432_max_difference_you_can_get_from_changing_an_integer/SolutionTest.java

use leetcode_in_rust::s1432::max_difference_you_can_get_from_changing_an_integer::Solution;

#[test]
fn test_max_diff() {
    assert_eq!(Solution::max_diff(555), 888);
}

#[test]
fn test_max_diff2() {
    assert_eq!(Solution::max_diff(9), 8);
}
