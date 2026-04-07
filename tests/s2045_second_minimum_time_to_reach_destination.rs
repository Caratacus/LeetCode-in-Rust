// Tests for Problem 2045: Second Minimum Time to Reach Destination
// Java reference: src/test/java/g2001_2100/s2045_second_minimum_time_to_reach_destination/SolutionTest.java

use leetcode_in_rust::s2045::second_minimum_time_to_reach_destination::Solution;

#[test]
fn test_second_minimum() {
    assert_eq!(
        Solution::second_minimum(
            5,
            vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![3, 4], vec![4, 5]],
            3,
            5
        ),
        13
    );
}

#[test]
fn test_second_minimum2() {
    assert_eq!(
        Solution::second_minimum(2, vec![vec![1, 2]], 3, 2),
        11
    );
}
