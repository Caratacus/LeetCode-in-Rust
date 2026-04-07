// Tests for Problem 2924: Find Champion II
// Java reference: src/test/java/g2901_3000/s2924_find_champion_ii/SolutionTest.java

use leetcode_in_rust::s2924::find_champion_ii::Solution;

#[test]
fn test_find_champion() {
    assert_eq!(Solution::find_champion(3, vec![vec![0, 1], vec![1, 2]]), 0);
}

#[test]
fn test_find_champion2() {
    assert_eq!(Solution::find_champion(4, vec![vec![0, 2], vec![1, 3], vec![1, 2]]), -1);
}
