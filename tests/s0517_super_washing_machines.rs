// Tests for Problem 0517: Super Washing Machines
// Java reference: src/test/java/g0501_0600/s0517_super_washing_machines/SolutionTest.java

use leetcode_in_rust::s0517::super_washing_machines::Solution;

#[test]
fn test_find_min_moves() {
    assert_eq!(Solution::find_min_moves(vec![1, 0, 5]), 3);
}

#[test]
fn test_find_min_moves2() {
    assert_eq!(Solution::find_min_moves(vec![0, 3, 0]), 2);
}
