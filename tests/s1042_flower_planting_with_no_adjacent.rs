// Tests for Problem 1042: Flower Planting With No Adjacent
// Java reference: src/test/java/g1001_1100/s1042_flower_planting_with_no_adjacent/SolutionTest.java

use leetcode_in_rust::s1042::flower_planting_with_no_adjacent::Solution;

#[test]
fn test_garden_no_adj() {
    assert_eq!(
        Solution::garden_no_adj(3, vec![vec![1, 2], vec![2, 3], vec![3, 1]]),
        vec![1, 2, 3]
    );
}

#[test]
fn test_garden_no_adj2() {
    assert_eq!(
        Solution::garden_no_adj(4, vec![vec![1, 2], vec![3, 4]]),
        vec![1, 2, 1, 2]
    );
}

#[test]
fn test_garden_no_adj3() {
    assert_eq!(
        Solution::garden_no_adj(4, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 1], vec![1, 3], vec![2, 4]]),
        vec![1, 2, 3, 4]
    );
}
