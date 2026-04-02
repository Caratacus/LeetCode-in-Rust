// Tests for Problem 1031: Maximum Sum of Two Non-Overlapping Subarrays
// Java reference: src/test/java/g1001_1100/s1031_maximum_sum_of_two_non_overlapping_subarrays/SolutionTest.java

use leetcode_in_rust::s1031::maximum_sum_of_two_non_overlapping_subarrays::Solution;

#[test]
fn test_max_sum_two_no_overlap() {
    assert_eq!(
        Solution::max_sum_two_no_overlap(vec![0, 6, 5, 2, 2, 5, 1, 9, 4], 1, 2),
        20
    );
}

#[test]
fn test_max_sum_two_no_overlap2() {
    assert_eq!(
        Solution::max_sum_two_no_overlap(vec![3, 8, 1, 3, 2, 1, 8, 9, 0], 3, 2),
        29
    );
}

#[test]
fn test_max_sum_two_no_overlap3() {
    assert_eq!(
        Solution::max_sum_two_no_overlap(vec![2, 1, 5, 6, 0, 9, 5, 0, 3, 8], 4, 3),
        31
    );
}
