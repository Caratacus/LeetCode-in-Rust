// Tests for Problem 2304: Minimum Path Cost in a Grid
// Java reference: src/test/java/g2301_2400/s2304_minimum_path_cost_in_a_grid/SolutionTest.java

use leetcode_in_rust::s2304::minimum_path_cost_in_a_grid::Solution;

#[test]
fn test_min_path_cost() {
    assert_eq!(
        Solution::min_path_cost(
            vec![vec![5, 3], vec![4, 0], vec![2, 1]],
            vec![
                vec![9, 8],
                vec![1, 5],
                vec![10, 12],
                vec![18, 6],
                vec![2, 4],
                vec![14, 3]
            ]
        ),
        17
    );
}

#[test]
fn test_min_path_cost2() {
    assert_eq!(
        Solution::min_path_cost(
            vec![vec![5, 1, 2], vec![4, 0, 3]],
            vec![
                vec![12, 10, 15],
                vec![20, 23, 8],
                vec![21, 7, 1],
                vec![8, 1, 13],
                vec![9, 10, 25],
                vec![5, 3, 2]
            ]
        ),
        6
    );
}
