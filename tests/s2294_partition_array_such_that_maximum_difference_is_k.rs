// Tests for Problem 2294: Partition Array Such That Maximum Difference Is K
// Java reference: src/test/java/g2201_2300/s2294_partition_array_such_that_maximum_difference_is_k/SolutionTest.java

use leetcode_in_rust::s2294::partition_array_such_that_maximum_difference_is_k::Solution;

#[test]
fn test_partition_array() {
    assert_eq!(Solution::partition_array(vec![3, 6, 1, 2, 5], 2), 2);
}

#[test]
fn test_partition_array2() {
    assert_eq!(Solution::partition_array(vec![1, 2, 3], 1), 2);
}

#[test]
fn test_partition_array3() {
    assert_eq!(Solution::partition_array(vec![2, 2, 4, 5], 0), 3);
}
