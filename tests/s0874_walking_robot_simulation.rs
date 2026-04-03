// Tests for Problem 0874: Walking Robot Simulation
// Java reference: src/test/java/g0801_0900/s0874_walking_robot_simulation/SolutionTest.java

use leetcode_in_rust::s0874::walking_robot_simulation::Solution;

#[test]
fn test_robot_sim() {
    assert_eq!(Solution::robot_sim(vec![4, -1, 3], vec![]), 25);
}

#[test]
fn test_robot_sim2() {
    assert_eq!(Solution::robot_sim(vec![4, -1, 4, -2, 4], vec![vec![2, 4]]), 65);
}

#[test]
fn test_robot_sim3() {
    assert_eq!(
        Solution::robot_sim(
            vec![4, -1, 4, -2, 4, 3, 5, -1, 3, -2, 4, -1, 3, -1, 2, -1, 4, -1, 5, -1, 3, -1, 2],
            vec![vec![2, 4], vec![4, 5], vec![2, 3], vec![0, 1], vec![1, 6]]
        ),
        202
    );
}
