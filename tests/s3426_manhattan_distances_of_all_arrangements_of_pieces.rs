// Tests for Problem 3426: Manhattan Distances of All Arrangements of Pieces
// Java reference: src/test/java/g3401_3500/s3426_manhattan_distances_of_all_arrangements_of_pieces/SolutionTest.java

use leetcode_in_rust::s3426::manhattan_distances_of_all_arrangements_of_pieces::Solution;

#[test]
fn test_distance_sum() {
    assert_eq!(Solution::distance_sum(2, 2, 2), 8);
}

#[test]
fn test_distance_sum2() {
    assert_eq!(Solution::distance_sum(1, 4, 3), 20);
}
