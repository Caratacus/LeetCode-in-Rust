// Tests for Problem 0455: Assign Cookies
// Java reference: src/test/java/g0401_0500/s0455_assign_cookies/SolutionTest.java

use leetcode_in_rust::s0455::assign_cookies::Solution;

#[test]
fn test_find_content_children() {
    assert_eq!(Solution::find_content_children(vec![1, 2, 3], vec![1, 1]), 1);
}

#[test]
fn test_find_content_children2() {
    assert_eq!(Solution::find_content_children(vec![1, 2], vec![1, 2, 3]), 2);
}
