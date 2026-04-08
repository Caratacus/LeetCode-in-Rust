// Tests for Problem 3285: Find Indices of Stable Mountains
// Java reference: src/test/java/g3201_3300/s3285_find_indices_of_stable_mountains/SolutionTest.java

use leetcode_in_rust::s3285::find_indices_of_stable_mountains::Solution;

#[test]
fn test_stable_mountains() {
    assert_eq!(Solution::stable_mountains(vec![1, 2, 3, 4, 5], 2), vec![3, 4]);
}

#[test]
fn test_stable_mountains2() {
    assert_eq!(Solution::stable_mountains(vec![10, 1, 10, 1, 10], 3), vec![1, 3]);
}

#[test]
fn test_stable_mountains3() {
    assert_eq!(Solution::stable_mountains(vec![10, 1, 10, 1, 10], 10), Vec::<i32>::new());
}
