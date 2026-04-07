// Tests for Problem 2923: Find Champion I
// Java reference: src/test/java/g2901_3000/s2923_find_champion_i/SolutionTest.java

use leetcode_in_rust::s2923::find_champion_i::Solution;

#[test]
fn test_find_champion() {
    assert_eq!(Solution::find_champion(vec![vec![0, 1], vec![0, 0]]), 0);
}

#[test]
fn test_find_champion2() {
    assert_eq!(Solution::find_champion(vec![vec![0, 0, 1], vec![1, 0, 1], vec![0, 0, 0]]), 1);
}

#[test]
fn test_find_champion3() {
    assert_eq!(Solution::find_champion(vec![vec![0, 0, 0], vec![1, 0, 0], vec![1, 1, 0]]), 2);
}
