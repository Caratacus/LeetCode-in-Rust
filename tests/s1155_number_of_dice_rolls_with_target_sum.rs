// Tests for Problem 1155: Number of Dice Rolls With Target Sum
// Java reference: src/test/java/g1101_1200/s1155_number_of_dice_rolls_with_target_sum/SolutionTest.java

use leetcode_in_rust::s1155::number_of_dice_rolls_with_target_sum::Solution;

#[test]
fn test_num_rolls_to_target() {
    assert_eq!(Solution::num_rolls_to_target(1, 6, 3), 1);
}

#[test]
fn test_num_rolls_to_target2() {
    assert_eq!(Solution::num_rolls_to_target(2, 6, 7), 6);
}

#[test]
fn test_num_rolls_to_target3() {
    assert_eq!(Solution::num_rolls_to_target(30, 30, 500), 222616187);
}
