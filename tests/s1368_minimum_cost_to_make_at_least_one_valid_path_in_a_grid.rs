// Tests for Problem 1368: Minimum Cost to Make at Least One Valid Path in a Grid
// Java reference: src/test/java/g1301_1400/s1368_minimum_cost_to_make_at_least_one_valid_path_in_a_grid/SolutionTest.java

use leetcode_in_rust::s1368::minimum_cost_to_make_at_least_one_valid_path_in_a_grid::Solution;

#[test]
fn test_min_cost() {
    let result = Solution::min_cost(vec![vec![1, 1, 1, 1], vec![2, 2, 2, 2], vec![1, 1, 1, 1], vec![2, 2, 2, 2]]);
    assert_eq!(result, 3);
}

#[test]
fn test_min_cost2() {
    let result = Solution::min_cost(vec![vec![1, 1, 3], vec![3, 2, 2], vec![1, 1, 4]]);
    assert_eq!(result, 0);
}

#[test]
fn test_min_cost3() {
    let result = Solution::min_cost(vec![vec![1, 2], vec![4, 3]]);
    assert_eq!(result, 1);
}
