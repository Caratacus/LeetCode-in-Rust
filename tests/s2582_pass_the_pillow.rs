// Tests for Problem 2582: Pass the Pillow
// Java reference: src/test/java/g2501_2600/s2582_pass_the_pillow/SolutionTest.java

use leetcode_in_rust::s2582::pass_the_pillow::Solution;

#[test]
fn test_pass_the_pillow() {
    assert_eq!(Solution::pass_the_pillow(4, 5), 2);
}

#[test]
fn test_pass_the_pillow2() {
    assert_eq!(Solution::pass_the_pillow(3, 2), 3);
}
