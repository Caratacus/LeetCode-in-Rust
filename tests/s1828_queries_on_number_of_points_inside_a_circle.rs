// Tests for Problem 1828: Queries on Number of Points Inside a Circle
// Java reference: src/test/java/g1801_1900/s1828_queries_on_number_of_points_inside_a_circle/SolutionTest.java

use leetcode_in_rust::s1828::queries_on_number_of_points_inside_a_circle::Solution;

#[test]
fn test_count_points() {
    assert_eq!(
        Solution::count_points(
            vec![vec![1, 3], vec![3, 3], vec![5, 3], vec![2, 2]],
            vec![vec![2, 3, 1], vec![4, 3, 1], vec![1, 1, 2]]
        ),
        vec![3, 2, 4]
    );
}

#[test]
fn test_count_points2() {
    assert_eq!(
        Solution::count_points(
            vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]],
            vec![vec![1, 2, 2], vec![2, 2, 2], vec![4, 3, 2], vec![4, 3, 3]]
        ),
        vec![2, 3, 2, 4]
    );
}
