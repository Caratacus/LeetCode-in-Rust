// Tests for Problem 1560: Most Visited Sector in a Circular Track
// Java reference: src/test/java/g1501_1600/s1560_most_visited_sector_in_a_circular_track/SolutionTest.java

use leetcode_in_rust::s1560::most_visited_sector_in_a_circular_track::Solution;

#[test]
fn test_most_visited() {
    let mut result = Solution::most_visited(4, vec![1, 3, 1, 2]);
    result.sort();
    assert_eq!(result, vec![1, 2]);
}

#[test]
fn test_most_visited2() {
    assert_eq!(Solution::most_visited(2, vec![2, 1, 2, 1, 2, 1, 2, 1, 2]), vec![2]);
}

#[test]
fn test_most_visited3() {
    let mut result = Solution::most_visited(7, vec![1, 3, 5, 7]);
    result.sort();
    assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7]);
}
