// Tests for Problem 1013: Partition Array Into Three Parts With Equal Sum
// Java reference: src/test/java/g1001_1100/s1013_partition_array_into_three_parts_with_equal_sum/SolutionTest.java

use leetcode_in_rust::s1013::partition_array_into_three_parts_with_equal_sum::Solution;

#[test]
fn test_can_three_parts_equal_sum() {
    assert_eq!(
        Solution::can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1]),
        true
    );
}

#[test]
fn test_can_three_parts_equal_sum2() {
    assert_eq!(
        Solution::can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1]),
        false
    );
}

#[test]
fn test_can_three_parts_equal_sum3() {
    assert_eq!(
        Solution::can_three_parts_equal_sum(vec![3, 3, 6, 5, -2, 2, 5, 1, -9, 4]),
        true
    );
}
