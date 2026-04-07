// Tests for Problem 2740: Find the Value of the Partition
// Java reference: src/test/java/g2701_2800/s2740_find_the_value_of_the_partition/SolutionTest.java

use leetcode_in_rust::s2740::find_the_value_of_the_partition::Solution;

#[test]
fn test_find_value_of_partition() {
    assert_eq!(Solution::find_value_of_partition(vec![1, 3, 2, 4]), 1);
}

#[test]
fn test_find_value_of_partition2() {
    assert_eq!(Solution::find_value_of_partition(vec![100, 1, 10]), 9);
}
