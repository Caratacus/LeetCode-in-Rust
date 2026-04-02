// Tests for Problem 0658: Find K Closest Elements
// Java reference: src/test/java/g0601_0700/s0658_find_k_closest_elements/SolutionTest.java

use leetcode_in_rust::s0658::find_k_closest_elements::Solution;

#[test]
fn test_find_closest_elements() {
    assert_eq!(
        Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3),
        vec![1, 2, 3, 4]
    );
}

#[test]
fn test_find_closest_elements2() {
    assert_eq!(
        Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, -1),
        vec![1, 2, 3, 4]
    );
}
