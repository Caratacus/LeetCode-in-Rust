// Tests for Problem 2963: Count the Number of Good Partitions
// Java reference: src/test/java/g2901_3000/s2963_count_the_number_of_good_partitions/SolutionTest.java

use leetcode_in_rust::s2963::count_the_number_of_good_partitions::Solution;

#[test]
fn test_number_of_good_partitions() {
    assert_eq!(Solution::number_of_good_partitions(vec![1, 2, 3, 4]), 8);
}

#[test]
fn test_number_of_good_partitions2() {
    assert_eq!(Solution::number_of_good_partitions(vec![1, 1, 1, 1]), 1);
}

#[test]
fn test_number_of_good_partitions3() {
    assert_eq!(Solution::number_of_good_partitions(vec![1, 2, 1, 3]), 2);
}
