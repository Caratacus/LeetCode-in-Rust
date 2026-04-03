// Tests for Problem 1493: Longest Subarray of 1's After Deleting One Element
// Java reference: src/test/java/g1401_1500/s1493_longest_subarray_of_1s_after_deleting_one_element/SolutionTest.java

use leetcode_in_rust::s1493::longest_subarray_of_1s_after_deleting_one_element::Solution;

#[test]
fn test_longest_subarray() {
    assert_eq!(Solution::longest_subarray(vec![1, 1, 0, 1]), 3);
}

#[test]
fn test_longest_subarray2() {
    assert_eq!(Solution::longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]), 5);
}

#[test]
fn test_longest_subarray3() {
    assert_eq!(Solution::longest_subarray(vec![1, 1, 1]), 2);
}
