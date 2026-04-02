// Tests for Problem 0027: Remove Element
// Java reference: src/test/java/g0001_0100/s0027_remove_element/SolutionTest.java

use leetcode_in_rust::s0027::remove_element::Solution;

#[test]
fn test_remove_element() {
    let mut nums = vec![3, 2, 2, 3];
    let len = Solution::remove_element(&mut nums, 3) as usize;
    assert_eq!(&nums[..len], &[2, 2]);
}

#[test]
fn test_remove_element2() {
    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let len = Solution::remove_element(&mut nums, 2) as usize;
    assert_eq!(&nums[..len], &[0, 1, 4, 0, 3]);
}
