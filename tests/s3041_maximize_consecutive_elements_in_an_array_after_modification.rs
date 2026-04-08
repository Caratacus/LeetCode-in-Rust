// Tests for Problem 3041: Maximize Consecutive Elements in an Array After Modification
// Java reference: src/test/java/g3001_3100/s3041_maximize_consecutive_elements_in_an_array_after_modification/SolutionTest.java

use leetcode_in_rust::s3041::maximize_consecutive_elements_in_an_array_after_modification::Solution;

#[test]
fn test_max_selected_elements() {
    assert_eq!(Solution::max_selected_elements(vec![2, 1, 5, 1, 1]), 3);
}

#[test]
fn test_max_selected_elements2() {
    assert_eq!(Solution::max_selected_elements(vec![1, 4, 7, 10]), 1);
}
