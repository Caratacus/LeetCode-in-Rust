// Tests for Problem 1866: Number of Ways to Rearrange Sticks With K Sticks Visible
// Java reference: src/test/java/g1801_1900/s1866_number_of_ways_to_rearrange_sticks_with_k_sticks_visible/SolutionTest.java

use leetcode_in_rust::s1866::number_of_ways_to_rearrange_sticks_with_k_sticks_visible::Solution;

#[test]
fn test_rearrange_sticks() {
    assert_eq!(Solution::rearrange_sticks(3, 2), 3);
}

#[test]
fn test_rearrange_sticks2() {
    assert_eq!(Solution::rearrange_sticks(5, 5), 1);
}

#[test]
fn test_rearrange_sticks3() {
    assert_eq!(Solution::rearrange_sticks(20, 11), 647427950);
}
