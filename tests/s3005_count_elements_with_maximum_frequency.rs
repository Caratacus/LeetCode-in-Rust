// Tests for Problem 3005: Count Elements With Maximum Frequency
// Java reference: src/test/java/g3001_3100/s3005_count_elements_with_maximum_frequency/SolutionTest.java

use leetcode_in_rust::s3005::count_elements_with_maximum_frequency::Solution;

#[test]
fn test_max_frequency_elements() {
    assert_eq!(Solution::max_frequency_elements(vec![1, 2, 2, 3, 1, 4]), 4);
}

#[test]
fn test_max_frequency_elements2() {
    assert_eq!(Solution::max_frequency_elements(vec![1, 2, 3, 4, 5]), 5);
}
