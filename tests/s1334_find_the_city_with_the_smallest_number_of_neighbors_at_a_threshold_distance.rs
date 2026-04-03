// Tests for Problem 1334: Find the City With the Smallest Number of Neighbors at a Threshold Distance
// Java reference: src/test/java/g1301_1400/s1334_find_the_city_with_the_smallest_number_of_neighbors_at_a_threshold_distance/SolutionTest.java

use leetcode_in_rust::s1334::find_the_city_with_the_smallest_number_of_neighbors_at_a_threshold_distance::Solution;

#[test]
fn test_find_the_city() {
    let result = Solution::find_the_city(4, vec![vec![0, 1, 3], vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 1]], 4);
    assert_eq!(result, 3);
}

#[test]
fn test_find_the_city2() {
    let result = Solution::find_the_city(5, vec![vec![0, 1, 2], vec![0, 4, 8], vec![1, 2, 3], vec![1, 4, 2], vec![2, 3, 1], vec![3, 4, 1]], 2);
    assert_eq!(result, 0);
}
