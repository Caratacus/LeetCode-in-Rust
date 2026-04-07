// Tests for Problem 2087: Minimum Cost Homecoming of a Robot in a Grid
// Java reference: src/test/java/g2001_2100/s2087_minimum_cost_homecoming_of_a_robot_in_a_grid/SolutionTest.java

use leetcode_in_rust::s2087::minimum_cost_homecoming_of_a_robot_in_a_grid::Solution;

#[test]
fn test_min_cost() {
    assert_eq!(
        Solution::min_cost(
            vec![1, 0],
            vec![2, 3],
            vec![5, 4, 3],
            vec![8, 2, 6, 7]
        ),
        18
    );
}

#[test]
fn test_min_cost2() {
    assert_eq!(
        Solution::min_cost(vec![0, 0], vec![0, 0], vec![5], vec![26]),
        0
    );
}
