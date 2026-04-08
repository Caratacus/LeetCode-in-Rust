// Tests for Problem 3206: Alternating Groups I
// Java reference: src/test/java/g3201_3300/s3206_alternating_groups_i/SolutionTest.java

use leetcode_in_rust::s3206::alternating_groups_i::Solution;

#[test]
fn test_number_of_alternating_groups() {
    assert_eq!(Solution::number_of_alternating_groups(vec![1, 1, 1]), 0);
}

#[test]
fn test_number_of_alternating_groups2() {
    assert_eq!(Solution::number_of_alternating_groups(vec![0, 1, 0, 0, 1]), 3);
}
