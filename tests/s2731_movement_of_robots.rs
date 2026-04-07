// Tests for Problem 2731: Movement of Robots
// Java reference: src/test/java/g2701_2800/s2731_movement_of_robots/SolutionTest.java

use leetcode_in_rust::s2731::movement_of_robots::Solution;

#[test]
fn test_sum_distance() {
    assert_eq!(Solution::sum_distance(vec![-2, 0, 2], "RLL".to_string(), 3), 8);
}

#[test]
fn test_sum_distance2() {
    assert_eq!(Solution::sum_distance(vec![1, 0], "RL".to_string(), 2), 5);
}
