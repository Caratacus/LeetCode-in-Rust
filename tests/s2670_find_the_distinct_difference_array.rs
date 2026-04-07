// Tests for Problem 2670: Find the Distinct Difference Array
// Java reference: src/test/java/g2601_2700/s2670_find_the_distinct_difference_array/SolutionTest.java

use leetcode_in_rust::s2670::find_the_distinct_difference_array::Solution;

#[test]
fn test_distinct_difference_array() {
    assert_eq!(
        Solution::distinct_difference_array(vec![1, 2, 3, 4, 5]),
        vec![-3, -1, 1, 3, 5]
    );
}

#[test]
fn test_distinct_difference_array2() {
    assert_eq!(
        Solution::distinct_difference_array(vec![3, 2, 3, 4, 2]),
        vec![-2, -1, 0, 2, 3]
    );
}
