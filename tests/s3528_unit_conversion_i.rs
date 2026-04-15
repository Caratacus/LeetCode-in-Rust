// Tests for Problem 3528: Unit Conversion I
// Java reference: src/test/java/g3501_3600/s3528_unit_conversion_i/SolutionTest.java

use leetcode_in_rust::s3528::unit_conversion_i::Solution;

#[test]
fn test_base_unit_conversions() {
    assert_eq!(Solution::base_unit_conversions(vec![vec![0, 1, 2], vec![1, 2, 3]]), vec![1, 2, 6]);
}

#[test]
fn test_base_unit_conversions2() {
    assert_eq!(
        Solution::base_unit_conversions(vec![vec![0, 1, 2], vec![0, 2, 3], vec![1, 3, 4], vec![1, 4, 5], vec![2, 5, 2], vec![4, 6, 3], vec![5, 7, 4]]),
        vec![1, 2, 3, 8, 10, 6, 30, 24]
    );
}
