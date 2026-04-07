// Tests for Problem 2122: Recover the Original Array
// Java reference: src/test/java/g2101_2200/s2122_recover_the_original_array/SolutionTest.java

use leetcode_in_rust::s2122::recover_the_original_array::Solution;

#[test]
fn test_recover_array() {
    let mut result = Solution::recover_array(vec![2, 10, 6, 4, 8, 12]);
    result.sort();
    assert_eq!(result, vec![3, 7, 11]);
}

#[test]
fn test_recover_array2() {
    let mut result = Solution::recover_array(vec![1, 1, 3, 3]);
    result.sort();
    assert_eq!(result, vec![2, 2]);
}

#[test]
fn test_recover_array3() {
    assert_eq!(Solution::recover_array(vec![5, 435]), vec![220]);
}
