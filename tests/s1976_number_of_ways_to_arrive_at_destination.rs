// Tests for Problem 1976: Number of Ways to Arrive at Destination
// Java reference: src/test/java/g1901_2000/s1976_number_of_ways_to_arrive_at_destination/SolutionTest.java

use leetcode_in_rust::s1976::number_of_ways_to_arrive_at_destination::Solution;

#[test]
fn test_count_paths() {
    assert_eq!(
        Solution::count_paths(
            7,
            vec![
                vec![0, 6, 7],
                vec![0, 1, 2],
                vec![1, 2, 3],
                vec![1, 3, 3],
                vec![6, 3, 3],
                vec![3, 5, 1],
                vec![6, 5, 1],
                vec![2, 5, 1],
                vec![0, 4, 5],
                vec![4, 6, 2],
            ]
        ),
        4
    );
}

#[test]
fn test_count_paths2() {
    assert_eq!(Solution::count_paths(2, vec![vec![1, 0, 10]]), 1);
}
