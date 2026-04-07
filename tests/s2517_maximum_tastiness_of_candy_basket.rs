// Tests for Problem 2517: Maximum Tastiness of Candy Basket
// Java reference: src/test/java/g2401_2500/s2517_maximum_tastiness_of_candy_basket/SolutionTest.java

use leetcode_in_rust::s2517::maximum_tastiness_of_candy_basket::Solution;

#[test]
fn test_maximum_tastiness() {
    assert_eq!(Solution::maximum_tastiness(vec![13, 5, 1, 8, 21, 2], 3), 8);
}

#[test]
fn test_maximum_tastiness2() {
    assert_eq!(Solution::maximum_tastiness(vec![1, 3, 1], 2), 2);
}

#[test]
fn test_maximum_tastiness3() {
    assert_eq!(Solution::maximum_tastiness(vec![7, 7, 7, 7], 2), 0);
}
