// Tests for Problem 2290: Minimum Obstacle Removal to Reach Corner
// Java reference: src/test/java/g2201_2300/s2290_minimum_obstacle_removal_to_reach_corner/SolutionTest.java

use leetcode_in_rust::s2290::minimum_obstacle_removal_to_reach_corner::Solution;

#[test]
fn test_minimum_obstacles() {
    assert_eq!(
        Solution::minimum_obstacles(vec![vec![0, 1, 1], vec![1, 1, 0], vec![1, 1, 0]]),
        2
    );
}

#[test]
fn test_minimum_obstacles2() {
    assert_eq!(
        Solution::minimum_obstacles(vec![
            vec![0, 1, 0, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 0, 0, 1, 0]
        ]),
        0
    );
}

#[test]
fn test_minimum_obstacles3() {
    assert_eq!(Solution::minimum_obstacles(vec![vec![1]]), 0);
}
