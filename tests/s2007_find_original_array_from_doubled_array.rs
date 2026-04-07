// Tests for Problem 2007: Find Original Array From Doubled Array
// Java reference: src/test/java/g2001_2100/s2007_find_original_array_from_doubled_array/SolutionTest.java

use leetcode_in_rust::s2007::find_original_array_from_doubled_array::Solution;

#[test]
fn test_find_original_array() {
    assert_eq!(
        Solution::find_original_array(vec![1, 3, 4, 2, 6, 8]),
        vec![1, 3, 4]
    );
}

#[test]
fn test_find_original_array2() {
    assert_eq!(Solution::find_original_array(vec![6, 3, 0, 1]), vec![] as Vec<i32>);
}

#[test]
fn test_find_original_array3() {
    assert_eq!(Solution::find_original_array(vec![1]), vec![] as Vec<i32>);
}
