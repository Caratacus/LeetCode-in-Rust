// Tests for Problem 3147: Taking Maximum Energy From the Mystic Dungeon
// Java reference: src/test/java/g3101_3200/s3147_taking_maximum_energy_from_the_mystic_dungeon/SolutionTest.java

use leetcode_in_rust::s3147::taking_maximum_energy_from_the_mystic_dungeon::Solution;
#[test]
fn test_maximum_energy() {
    assert_eq!(Solution::maximum_energy(vec![5, 2, -10, -5, 1], 3), 3);
}
#[test]
fn test_maximum_energy2() {
    assert_eq!(Solution::maximum_energy(vec![-2, -3, -1], 2), -1);
}
