// Tests for Problem 2161: Partition Array According to Given Pivot
// Java reference: src/test/java/g2101_2200/s2161_partition_array_according_to_given_pivot/SolutionTest.java

use leetcode_in_rust::s2161::partition_array_according_to_given_pivot::Solution;

#[test]
fn test_pivot_array() {
    assert_eq!(
        Solution::pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10),
        vec![9, 5, 3, 10, 10, 12, 14]
    );
}

#[test]
fn test_pivot_array2() {
    assert_eq!(Solution::pivot_array(vec![-3, 4, 3, 2], 2), vec![-3, 2, 4, 3]);
}
