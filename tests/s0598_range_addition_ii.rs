// Tests for Problem 0598: Range Addition II
// Java reference: src/test/java/g0501_0600/s0598_range_addition_ii/SolutionTest.java

use leetcode_in_rust::s0598::range_addition_ii::Solution;

#[test]
fn test_max_count() {
    assert_eq!(
        Solution::max_count(3, 3, vec![vec![2, 2], vec![3, 3]]),
        4
    );
}

#[test]
fn test_max_count2() {
    assert_eq!(
        Solution::max_count(
            3,
            3,
            vec![
                vec![2, 2], vec![3, 3], vec![3, 3], vec![3, 3],
                vec![2, 2], vec![3, 3], vec![3, 3], vec![3, 3],
                vec![2, 2], vec![3, 3], vec![3, 3], vec![3, 3]
            ]
        ),
        4
    );
}

#[test]
fn test_max_count3() {
    assert_eq!(Solution::max_count(3, 3, vec![]), 9);
}
