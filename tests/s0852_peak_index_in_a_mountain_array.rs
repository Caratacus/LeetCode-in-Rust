// Tests for Problem 0852: Peak Index in a Mountain Array
// Java reference: src/test/java/g0801_0900/s0852_peak_index_in_a_mountain_array/SolutionTest.java

use leetcode_in_rust::s0852::peak_index_in_a_mountain_array::Solution;

#[test]
fn test_peak_index_in_mountain_array() {
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 0]), 1);
}

#[test]
fn test_peak_index_in_mountain_array2() {
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
}

#[test]
fn test_peak_index_in_mountain_array3() {
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 10, 5, 2]), 1);
}

#[test]
fn test_peak_index_in_mountain_array4() {
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 2, 3, 2, 1]), 3);
}

#[test]
fn test_peak_index_in_mountain_array5() {
    assert_eq!(Solution::peak_index_in_mountain_array(vec![5, 10, 7]), 1);
}

#[test]
fn test_peak_index_in_mountain_array6() {
    assert_eq!(Solution::peak_index_in_mountain_array(vec![5, 4, 3, 2, 1]), 1);
}

#[test]
fn test_peak_index_in_mountain_array7() {
    assert_eq!(Solution::peak_index_in_mountain_array(vec![1, 2, 3, 4, 5]), -1);
}

#[test]
fn test_peak_index_in_mountain_array8() {
    assert_eq!(Solution::peak_index_in_mountain_array(vec![3, 3, 3, 3]), -1);
}
