// Tests for Problem 2585: Number of Ways to Earn Points
// Java reference: src/test/java/g2501_2600/s2585_number_of_ways_to_earn_points/SolutionTest.java

use leetcode_in_rust::s2585::number_of_ways_to_earn_points::Solution;

#[test]
fn test_ways_to_reach_target() {
    assert_eq!(
        Solution::ways_to_reach_target(6, vec![vec![6, 1], vec![3, 2], vec![2, 3]]),
        7
    );
}

#[test]
fn test_ways_to_reach_target2() {
    assert_eq!(
        Solution::ways_to_reach_target(5, vec![vec![50, 1], vec![50, 2], vec![50, 5]]),
        4
    );
}
