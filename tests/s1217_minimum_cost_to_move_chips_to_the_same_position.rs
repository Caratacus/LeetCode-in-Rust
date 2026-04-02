// Tests for Problem 1217: Minimum Cost to Move Chips to The Same Position
// Java reference: src/test/java/g1201_1300/s1217_minimum_cost_to_move_chips_to_the_same_position/SolutionTest.java

use leetcode_in_rust::s1217::minimum_cost_to_move_chips_to_the_same_position::Solution;

#[test]
fn test_min_cost_to_move_chips() {
    assert_eq!(Solution::min_cost_to_move_chips(vec![1, 2, 3]), 1);
}

#[test]
fn test_min_cost_to_move_chips2() {
    assert_eq!(Solution::min_cost_to_move_chips(vec![2, 2, 2, 3, 3]), 2);
}

#[test]
fn test_min_cost_to_move_chips3() {
    assert_eq!(Solution::min_cost_to_move_chips(vec![1, 1000000000]), 1);
}
