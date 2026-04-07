// Tests for Problem 1855: Maximum Distance Between a Pair of Values
// Java reference: src/test/java/g1801_1900/s1855_maximum_distance_between_a_pair_of_values/SolutionTest.java

use leetcode_in_rust::s1855::maximum_distance_between_a_pair_of_values::Solution;

#[test]
fn test_max_distance() {
    assert_eq!(
        Solution::max_distance(vec![55, 30, 5, 4, 2], vec![100, 20, 10, 10, 5]),
        2
    );
}

#[test]
fn test_max_distance2() {
    assert_eq!(Solution::max_distance(vec![2, 2, 2], vec![10, 10, 1]), 1);
}

#[test]
fn test_max_distance3() {
    assert_eq!(
        Solution::max_distance(vec![30, 29, 19, 5], vec![25, 25, 25, 25, 25]),
        2
    );
}
