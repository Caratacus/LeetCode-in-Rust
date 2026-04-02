// Tests for Problem 0057: Insert Interval
// Java reference: src/test/java/g0001_0100/s0057_insert_interval/SolutionTest.java

use leetcode_in_rust::s0057::insert_interval::Solution;

#[test]
fn test_insert() {
    let intervals = vec![vec![1, 3], vec![6, 9]];
    let new_interval = vec![2, 5];
    let result = Solution::insert(intervals, new_interval);
    assert_eq!(result, vec![vec![1, 5], vec![6, 9]]);
}

#[test]
fn test_insert2() {
    let intervals = vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]];
    let new_interval = vec![4, 8];
    let result = Solution::insert(intervals, new_interval);
    assert_eq!(result, vec![vec![1, 2], vec![3, 10], vec![12, 16]]);
}
