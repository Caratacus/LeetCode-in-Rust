// Tests for Problem 2025: Maximum Number of Ways to Partition an Array
// Java reference: src/test/java/g2001_2100/s2025_maximum_number_of_ways_to_partition_an_array/SolutionTest.java

use leetcode_in_rust::s2025::maximum_number_of_ways_to_partition_an_array::Solution;

#[test]
fn test_ways_to_partition() {
    assert_eq!(Solution::ways_to_partition(vec![2, -1, 2], 3), 1);
}

#[test]
fn test_ways_to_partition2() {
    assert_eq!(Solution::ways_to_partition(vec![0, 0, 0], 1), 2);
}

#[test]
fn test_ways_to_partition3() {
    assert_eq!(
        Solution::ways_to_partition(
            vec![22, 4, -25, -20, -15, 15, -16, 7, 19, -10, 0, -13, -14],
            -33
        ),
        4
    );
}
