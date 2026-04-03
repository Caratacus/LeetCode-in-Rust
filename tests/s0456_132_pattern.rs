// Tests for Problem 0456: 132 Pattern
// Java reference: src/test/java/g0401_0500/s0456_132_pattern/SolutionTest.java

use leetcode_in_rust::s0456::p132_pattern::Solution;

#[test]
fn test_find132pattern() {
    // nums=[1,2,3,4]
    assert_eq!(Solution::find132pattern(vec![1, 2, 3, 4]), false);
}

#[test]
fn test_find132pattern2() {
    // nums=[3,1,4,2]
    assert_eq!(Solution::find132pattern(vec![3, 1, 4, 2]), true);
}

#[test]
fn test_find132pattern3() {
    // nums=[-1,3,2,0]
    assert_eq!(Solution::find132pattern(vec![-1, 3, 2, 0]), true);
}
