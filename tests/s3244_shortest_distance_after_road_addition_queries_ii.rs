// Tests for Problem 3244: Shortest Distance After Road Addition Queries II
// Java reference: src/test/java/g3201_3300/s3244_shortest_distance_after_road_addition_queries_ii/SolutionTest.java

use leetcode_in_rust::s3244::shortest_distance_after_road_addition_queries_ii::Solution;

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
