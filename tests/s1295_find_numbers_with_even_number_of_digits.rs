// Tests for Problem 1295: Find Numbers with Even Number of Digits
// Java reference: src/test/java/g1201_1300/s1295_find_numbers_with_even_number_of_digits/SolutionTest.java

use leetcode_in_rust::s1295::find_numbers_with_even_number_of_digits::Solution;

#[test]
fn test_find_numbers() {
    assert_eq!(Solution::find_numbers(vec![12, 345, 2, 6, 7896]), 2);
}

#[test]
fn test_find_numbers2() {
    assert_eq!(Solution::find_numbers(vec![555, 901, 482, 1771]), 1);
}
