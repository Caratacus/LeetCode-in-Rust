// Tests for Problem 0638: Shopping Offers
// Java reference: src/test/java/g0601_0700/s0638_shopping_offers/SolutionTest.java

use leetcode_in_rust::s0638::shopping_offers::Solution;

#[test]
fn test_shopping_offers() {
    let price = vec![2, 5];
    let special = vec![vec![3, 0, 5], vec![1, 2, 10]];
    let needs = vec![3, 2];
    assert_eq!(Solution::shopping_offers(price, special, needs), 14);
}

#[test]
fn test_shopping_offers2() {
    let price = vec![2, 3, 4];
    let special = vec![vec![1, 1, 0, 4], vec![2, 2, 1, 9]];
    let needs = vec![1, 2, 1];
    assert_eq!(Solution::shopping_offers(price, special, needs), 11);
}
