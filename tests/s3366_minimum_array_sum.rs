// Tests for Problem 3366: Minimum Array Sum
// Java reference: src/test/java/g3301_3400/s3366_minimum_array_sum/SolutionTest.java

use leetcode_in_rust::s3366::minimum_array_sum::Solution;

#[test]
fn test_min_array_sum() {
    assert_eq!(Solution::min_array_sum(vec![2, 8, 3, 19, 3], 3, 1, 1), 23);
}

#[test]
fn test_min_array_sum2() {
    assert_eq!(Solution::min_array_sum(vec![2, 4, 3], 3, 2, 1), 3);
}

#[test]
fn test_min_array_sum3() {
    assert_eq!(
        Solution::min_array_sum(
            vec![1, 3, 5, 7, 9, 12, 12, 12, 13, 15, 15, 15, 16, 17, 19, 20],
            11,
            15,
            4,
        ),
        77
    );
}
