// Tests for Problem 2574: Left and Right Sum Differences
// Java reference: src/test/java/g2501_2600/s2574_left_and_right_sum_differences/SolutionTest.java

use leetcode_in_rust::s2574::left_and_right_sum_differences::Solution;

#[test]
fn test_left_right_difference() {
    assert_eq!(
        Solution::left_right_difference(vec![10, 4, 8, 3]),
        vec![15, 1, 11, 22]
    );
}

#[test]
fn test_left_right_difference2() {
    assert_eq!(Solution::left_right_difference(vec![1]), vec![0]);
}
