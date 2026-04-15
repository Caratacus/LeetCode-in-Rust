// Tests for Problem 3396: Minimum Number of Operations to Make Elements in Array Distinct
// Java reference: src/test/java/g3301_3400/s3396_minimum_number_of_operations_to_make_elements_in_array_distinct/SolutionTest.java

use leetcode_in_rust::s3396::minimum_number_of_operations_to_make_elements_in_array_distinct::Solution;

#[test]
fn test_minimum_operations() {
    assert_eq!(
        Solution::minimum_operations(vec![1, 2, 3, 4, 2, 3, 3, 5, 7]),
        2
    );
}

#[test]
fn test_minimum_operations2() {
    assert_eq!(Solution::minimum_operations(vec![4, 5, 6, 4, 4]), 2);
}

#[test]
fn test_minimum_operations3() {
    assert_eq!(Solution::minimum_operations(vec![6, 7, 8, 9]), 0);
}
