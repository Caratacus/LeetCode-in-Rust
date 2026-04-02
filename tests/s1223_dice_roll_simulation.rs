// Tests for Problem 1223: Dice Roll Simulation
// Java reference: src/test/java/g1201_1300/s1223_dice_roll_simulation/SolutionTest.java

use leetcode_in_rust::s1223::dice_roll_simulation::Solution;

#[test]
fn test_die_simulator() {
    assert_eq!(Solution::die_simulator(2, vec![1, 1, 2, 2, 2, 3]), 34);
}

#[test]
fn test_die_simulator2() {
    assert_eq!(Solution::die_simulator(2, vec![1, 1, 1, 1, 1, 1]), 30);
}

#[test]
fn test_die_simulator3() {
    assert_eq!(Solution::die_simulator(3, vec![1, 1, 1, 2, 2, 3]), 181);
}
