// Tests for Problem 1293: Shortest Path in a Grid with Obstacles Elimination
// Java reference: src/test/java/g1201_1300/s1293_shortest_path_in_a_grid_with_obstacles_elimination/SolutionTest.java

use leetcode_in_rust::s1293::shortest_path_in_a_grid_with_obstacles_elimination::Solution;

#[test]
fn test_shortest_path() {
    assert_eq!(
        Solution::shortest_path(
            vec![
                vec![0, 0, 0],
                vec![1, 1, 0],
                vec![0, 0, 0],
                vec![0, 1, 1],
                vec![0, 0, 0]
            ],
            1
        ),
        6
    );
}

#[test]
fn test_shortest_path2() {
    assert_eq!(
        Solution::shortest_path(vec![vec![0, 1, 1], vec![1, 1, 1], vec![1, 0, 0]], 1),
        -1
    );
}
