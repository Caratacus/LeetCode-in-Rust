// Tests for Problem 0436: Find Right Interval
// Java reference: src/test/java/g0401_0500/s0436_find_right_interval/SolutionTest.java

use leetcode_in_rust::s0436::find_right_interval::Solution;

#[test]
fn test_find_right_interval() {
    assert_eq!(
        Solution::find_right_interval(vec![vec![1, 2]]),
        vec![-1]
    );
}

#[test]
fn test_find_right_interval2() {
    assert_eq!(
        Solution::find_right_interval(vec![vec![3, 4], vec![2, 3], vec![1, 2]]),
        vec![-1, 0, 1]
    );
}
