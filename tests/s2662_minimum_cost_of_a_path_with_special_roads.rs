// Tests for Problem 2662: Minimum Cost of a Path With Special Roads
// Java reference: src/test/java/g2601_2700/s2662_minimum_cost_of_a_path_with_special_roads/SolutionTest.java

use leetcode_in_rust::s2662::minimum_cost_of_a_path_with_special_roads::Solution;

#[test]
fn test_minimum_cost() {
    assert_eq!(
        Solution::minimum_cost(
            vec![1, 1],
            vec![4, 5],
            vec![vec![1, 2, 3, 3, 2], vec![3, 4, 4, 5, 1]]
        ),
        5
    );
}

#[test]
fn test_minimum_cost2() {
    assert_eq!(
        Solution::minimum_cost(
            vec![3, 2],
            vec![5, 7],
            vec![
                vec![3, 2, 3, 4, 4],
                vec![3, 3, 5, 5, 5],
                vec![3, 4, 5, 6, 6]
            ]
        ),
        7
    );
}
