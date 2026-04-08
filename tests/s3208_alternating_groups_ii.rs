// Tests for Problem 3208: Alternating Groups II
// Java reference: src/test/java/g3201_3300/s3208_alternating_groups_ii/SolutionTest.java

use leetcode_in_rust::s3208::alternating_groups_ii::Solution;

#[test]
fn test_number_of_alternating_groups() {
    assert_eq!(Solution::number_of_alternating_groups(vec![0, 1, 0, 1, 0], 3), 3);
}

#[test]
fn test_number_of_alternating_groups2() {
    assert_eq!(Solution::number_of_alternating_groups(vec![0, 1, 0, 0, 1, 0, 1], 6), 2);
}

#[test]
fn test_number_of_alternating_groups3() {
    assert_eq!(Solution::number_of_alternating_groups(vec![1, 1, 0, 1], 4), 0);
}
