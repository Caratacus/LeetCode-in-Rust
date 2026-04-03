// Tests for Problem 1562: Find Latest Group of Size M
// Java reference: src/test/java/g1501_1600/s1562_find_latest_group_of_size_m/SolutionTest.java

use leetcode_in_rust::s1562::find_latest_group_of_size_m::Solution;

#[test]
fn test_find_latest_step() {
    assert_eq!(Solution::find_latest_step(vec![3, 5, 1, 2, 4], 1), 4);
}

#[test]
fn test_find_latest_step2() {
    assert_eq!(Solution::find_latest_step(vec![3, 1, 5, 4, 2], 2), -1);
}
