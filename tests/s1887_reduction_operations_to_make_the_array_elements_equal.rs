// Tests for Problem 1887: Reduction Operations to Make the Array Elements Equal
// Java reference: src/test/java/g1801_1900/s1887_reduction_operations_to_make_the_array_elements_equal/SolutionTest.java

use leetcode_in_rust::s1887::reduction_operations_to_make_the_array_elements_equal::Solution;

#[test]
fn test_reduction_operations() {
    assert_eq!(Solution::reduction_operations(vec![5, 1, 3]), 3);
}

#[test]
fn test_reduction_operations2() {
    assert_eq!(Solution::reduction_operations(vec![1, 1, 1]), 0);
}

#[test]
fn test_reduction_operations3() {
    assert_eq!(Solution::reduction_operations(vec![1, 1, 2, 2, 3]), 4);
}
