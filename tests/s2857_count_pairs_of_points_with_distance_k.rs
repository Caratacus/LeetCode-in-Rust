// Tests for Problem 2857: Count Pairs of Points With Distance k
// Java reference: src/test/java/g2801_2900/s2857_count_pairs_of_points_with_distance_k/SolutionTest.java

use leetcode_in_rust::s2857::count_pairs_of_points_with_distance_k::Solution;

#[test]
fn test_count_pairs() {
    assert_eq!(
        Solution::count_pairs(vec![vec![1, 2], vec![4, 2], vec![1, 3], vec![5, 2]], 5),
        2
    );
}

#[test]
fn test_count_pairs2() {
    assert_eq!(
        Solution::count_pairs(vec![vec![1, 3], vec![1, 3], vec![1, 3], vec![1, 3], vec![1, 3]], 0),
        10
    );
}
