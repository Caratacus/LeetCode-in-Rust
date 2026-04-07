// Tests for Problem 2202: Maximize the Topmost Element After K Moves
// Java reference: src/test/java/g2201_2300/s2202_maximize_the_topmost_element_after_k_moves/SolutionTest.java

use leetcode_in_rust::s2202::maximize_the_topmost_element_after_k_moves::Solution;

#[test]
fn test_maximum_top() {
    assert_eq!(Solution::maximum_top(vec![5, 2, 2, 4, 0, 6], 4), 5);
}

#[test]
fn test_maximum_top2() {
    assert_eq!(Solution::maximum_top(vec![2], 1), -1);
}

#[test]
fn test_maximum_top3() {
    assert_eq!(Solution::maximum_top(vec![3], 0), 3);
}
