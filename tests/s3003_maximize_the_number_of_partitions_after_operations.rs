// Tests for Problem 3003: Maximize the Number of Partitions After Operations
// Java reference: src/test/java/g3001_3100/s3003_maximize_the_number_of_partitions_after_operations/SolutionTest.java

use leetcode_in_rust::s3003::maximize_the_number_of_partitions_after_operations::Solution;

#[test]
fn test_max_partitions_after_operations() {
    assert_eq!(Solution::max_partitions_after_operations("accca".to_string(), 2), 3);
}

#[test]
fn test_max_partitions_after_operations2() {
    assert_eq!(Solution::max_partitions_after_operations("aabaab".to_string(), 3), 1);
}

#[test]
fn test_max_partitions_after_operations3() {
    assert_eq!(Solution::max_partitions_after_operations("xxyz".to_string(), 1), 4);
}
