// Tests for Problem 2518: Number of Great Partitions
// Java reference: src/test/java/g2401_2500/s2518_number_of_great_partitions/SolutionTest.java

use leetcode_in_rust::s2518::number_of_great_partitions::Solution;

#[test]
fn test_count_partitions() {
    assert_eq!(Solution::count_partitions(vec![1, 2, 3, 4], 4), 6);
}

#[test]
fn test_count_partitions2() {
    assert_eq!(Solution::count_partitions(vec![3, 3, 3], 4), 0);
}

#[test]
fn test_count_partitions3() {
    assert_eq!(Solution::count_partitions(vec![6, 6], 2), 2);
}
