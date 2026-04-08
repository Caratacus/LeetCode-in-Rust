// Tests for Problem 3243: Shortest Distance After Road Addition Queries I
// Java reference: src/test/java/g3201_3300/s3243_shortest_distance_after_road_addition_queries_i/SolutionTest.java

use leetcode_in_rust::s3243::shortest_distance_after_road_addition_queries_i::Solution;

#[test]
fn test_shortest_distance_after_queries() {
    assert_eq!(
        Solution::shortest_distance_after_queries(5, vec![vec![2, 4], vec![0, 2], vec![0, 4]]),
        vec![3, 2, 1]
    );
}

#[test]
fn test_shortest_distance_after_queries2() {
    assert_eq!(
        Solution::shortest_distance_after_queries(4, vec![vec![0, 3], vec![0, 2]]),
        vec![1, 1]
    );
}
