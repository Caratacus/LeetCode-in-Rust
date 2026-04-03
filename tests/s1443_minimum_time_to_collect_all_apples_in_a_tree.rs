// Tests for Problem 1443: Minimum Time to Collect All Apples in a Tree
// Java reference: src/test/java/g1401_1500/s1443_minimum_time_to_collect_all_apples_in_a_tree/SolutionTest.java

use leetcode_in_rust::s1443::minimum_time_to_collect_all_apples_in_a_tree::Solution;

#[test]
fn test_min_time() {
    let edges = vec![vec![0, 1], vec![0, 2], vec![1, 4], vec![1, 5], vec![2, 3], vec![2, 6]];
    let has_apple = vec![false, false, true, false, true, true, false];
    assert_eq!(Solution::min_time(7, edges, has_apple), 8);
}

#[test]
fn test_min_time2() {
    let edges = vec![vec![0, 1], vec![0, 2], vec![1, 4], vec![1, 5], vec![2, 3], vec![2, 6]];
    let has_apple = vec![false, false, true, false, false, true, false];
    assert_eq!(Solution::min_time(7, edges, has_apple), 6);
}

#[test]
fn test_min_time3() {
    let edges = vec![vec![0, 1], vec![0, 2], vec![1, 4], vec![1, 5], vec![2, 3], vec![2, 6]];
    let has_apple = vec![false, false, false, false, false, false, false];
    assert_eq!(Solution::min_time(7, edges, has_apple), 0);
}
