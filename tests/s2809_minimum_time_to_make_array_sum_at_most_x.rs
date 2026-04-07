// Tests for Problem 2809: Minimum Time to Make Array Sum At Most X
// Java reference: src/test/java/g2701_2800/s2809_minimum_time_to_make_array_sum_at_most_x/SolutionTest.java

use leetcode_in_rust::s2809::minimum_time_to_make_array_sum_at_most_x::Solution;

#[test]
fn test_minimum_time() {
    assert_eq!(Solution::minimum_time(vec![1, 2, 3], vec![1, 2, 3], 4), 3);
}

#[test]
fn test_minimum_time2() {
    assert_eq!(Solution::minimum_time(vec![1, 2, 3], vec![3, 3, 3], 4), -1);
}
