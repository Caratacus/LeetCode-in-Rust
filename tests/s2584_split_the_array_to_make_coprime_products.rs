// Tests for Problem 2584: Split the Array to Make Coprime Products
// Java reference: src/test/java/g2501_2600/s2584_split_the_array_to_make_coprime_products/SolutionTest.java

use leetcode_in_rust::s2584::split_the_array_to_make_coprime_products::Solution;

#[test]
fn test_find_valid_split() {
    assert_eq!(Solution::find_valid_split(vec![4, 7, 8, 15, 3, 5]), 2);
}

#[test]
fn test_find_valid_split2() {
    assert_eq!(Solution::find_valid_split(vec![4, 7, 15, 8, 3, 5]), -1);
}
