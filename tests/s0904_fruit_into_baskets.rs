// Tests for Problem 0904: Fruit Into Baskets
// Java reference: src/test/java/g0901_1000/s0904_fruit_into_baskets/SolutionTest.java

use leetcode_in_rust::s0904::fruit_into_baskets::Solution;

#[test]
fn test_total_fruit() {
    assert_eq!(Solution::total_fruit(vec![1, 2, 1]), 3);
}

#[test]
fn test_total_fruit2() {
    assert_eq!(Solution::total_fruit(vec![0, 1, 2, 2]), 3);
}

#[test]
fn test_total_fruit3() {
    assert_eq!(Solution::total_fruit(vec![1, 2, 3, 2, 2]), 4);
}
