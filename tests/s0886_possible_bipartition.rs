// Tests for Problem 0886: Possible Bipartition
// Java reference: src/test/java/g0801_0900/s0886_possible_bipartition/SolutionTest.java

use leetcode_in_rust::s0886::possible_bipartition::Solution;

#[test]
fn test_possible_bipartition() {
    let result = Solution::possible_bipartition(4, vec![vec![1, 2], vec![1, 3], vec![2, 4]]);
    assert_eq!(result, true);
}

#[test]
fn test_possible_bipartition2() {
    let result = Solution::possible_bipartition(3, vec![vec![1, 2], vec![1, 3], vec![2, 3]]);
    assert_eq!(result, false);
}

#[test]
fn test_possible_bipartition3() {
    let result = Solution::possible_bipartition(5, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]]);
    assert_eq!(result, false);
}
