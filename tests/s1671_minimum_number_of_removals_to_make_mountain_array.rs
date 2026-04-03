// Tests for Problem 1671: Minimum Number of Removals to Make Mountain Array
// Java reference: src/test/java/g1601_1700/s1671_minimum_number_of_removals_to_make_mountain_array/SolutionTest.java

use leetcode_in_rust::s1671::minimum_number_of_removals_to_make_mountain_array::Solution;

#[test]
fn test_minimum_mountain_removals() {
    assert_eq!(Solution::minimum_mountain_removals(vec![1, 3, 1]), 0);
}

#[test]
fn test_minimum_mountain_removals2() {
    assert_eq!(Solution::minimum_mountain_removals(vec![2, 1, 1, 5, 6, 2, 3, 1]), 3);
}
