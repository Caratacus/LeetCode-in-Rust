// Tests for Problem 2478: Number of Beautiful Partitions
// Java reference: src/test/java/g2401_2500/s2478_number_of_beautiful_partitions/SolutionTest.java

use leetcode_in_rust::s2478::number_of_beautiful_partitions::Solution;

#[test]
fn test_beautiful_partitions() {
    assert_eq!(Solution::beautiful_partitions("23542185131".to_string(), 3, 3), 1);
}

#[test]
fn test_beautiful_partitions2() {
    assert_eq!(Solution::beautiful_partitions("23542185131".to_string(), 3, 2), 3);
}

#[test]
fn test_beautiful_partitions3() {
    assert_eq!(Solution::beautiful_partitions("3312958".to_string(), 3, 1), 1);
}
