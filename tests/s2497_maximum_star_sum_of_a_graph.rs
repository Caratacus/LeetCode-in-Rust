// Tests for Problem 2497: Maximum Star Sum of a Graph
// Java reference: src/test/java/g2401_2500/s2497_maximum_star_sum_of_a_graph/SolutionTest.java

use leetcode_in_rust::s2497::maximum_star_sum_of_a_graph::Solution;

#[test]
fn test_max_star_sum() {
    assert_eq!(
        Solution::max_star_sum(
            vec![1, 2, 3, 4, 10, -10, -20],
            vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4], vec![3, 5], vec![3, 6]],
            2
        ),
        16
    );
}

#[test]
fn test_max_star_sum2() {
    assert_eq!(
        Solution::max_star_sum(vec![-5], vec![], 0),
        -5
    );
}
