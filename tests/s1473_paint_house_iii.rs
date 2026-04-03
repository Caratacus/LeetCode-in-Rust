// Tests for Problem 1473: Paint House III
// Java reference: src/test/java/g1401_1500/s1473_paint_house_iii/SolutionTest.java

use leetcode_in_rust::s1473::paint_house_iii::Solution;

#[test]
fn test_min_cost() {
    let houses = vec![0, 0, 0, 0, 0];
    let cost = vec![vec![1, 10], vec![10, 1], vec![10, 1], vec![1, 10], vec![5, 1]];
    assert_eq!(Solution::min_cost(houses, cost, 2, 3), 9);
}

#[test]
fn test_min_cost2() {
    let houses = vec![0, 2, 1, 2, 0];
    let cost = vec![vec![1, 10], vec![10, 1], vec![10, 1], vec![1, 10], vec![5, 1]];
    assert_eq!(Solution::min_cost(houses, cost, 2, 3), 11);
}

#[test]
fn test_min_cost3() {
    let houses = vec![3, 1, 2, 3];
    let cost = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
    assert_eq!(Solution::min_cost(houses, cost, 3, 3), -1);
}
