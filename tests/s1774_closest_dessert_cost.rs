// Tests for Problem 1774: Closest Dessert Cost
// Java reference: src/test/java/g1701_1800/s1774_closest_dessert_cost/SolutionTest.java

use leetcode_in_rust::s1774::closest_dessert_cost::Solution;

#[test]
fn test_closest_cost() {
    assert_eq!(Solution::closest_cost(vec![1, 7], vec![3, 4], 10), 10);
}

#[test]
fn test_closest_cost2() {
    assert_eq!(Solution::closest_cost(vec![2, 3], vec![4, 5, 100], 18), 17);
}

#[test]
fn test_closest_cost3() {
    assert_eq!(Solution::closest_cost(vec![3, 10], vec![2, 5], 9), 8);
}
