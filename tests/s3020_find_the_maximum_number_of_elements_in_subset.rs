// Tests for Problem 3020: Find the Maximum Number of Elements in Subset
// Java reference: src/test/java/g3001_3100/s3020_find_the_maximum_number_of_elements_in_subset/SolutionTest.java

use leetcode_in_rust::s3020::find_the_maximum_number_of_elements_in_subset::Solution;

#[test]
fn test_maximum_length() {
    assert_eq!(Solution::maximum_length(vec![5, 4, 1, 2, 2]), 3);
}

#[test]
fn test_maximum_length2() {
    assert_eq!(Solution::maximum_length(vec![1, 3, 2, 4]), 1);
}
