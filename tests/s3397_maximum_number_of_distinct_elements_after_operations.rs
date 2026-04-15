// Tests for Problem 3397: Maximum Number of Distinct Elements After Operations
// Java reference: src/test/java/g3301_3400/s3397_maximum_number_of_distinct_elements_after_operations/SolutionTest.java

use leetcode_in_rust::s3397::maximum_number_of_distinct_elements_after_operations::Solution;

#[test]
fn test_max_distinct_elements() {
    assert_eq!(Solution::max_distinct_elements(vec![1, 2, 2, 3, 3, 4], 2), 6);
}

#[test]
fn test_max_distinct_elements2() {
    assert_eq!(Solution::max_distinct_elements(vec![4, 4, 4, 4], 1), 3);
}
