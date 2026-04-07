// Tests for Problem 2698: Find the Punishment Number of an Integer
// Java reference: src/test/java/g2601_2700/s2698_find_the_punishment_number_of_an_integer/SolutionTest.java

use leetcode_in_rust::s2698::find_the_punishment_number_of_an_integer::Solution;

#[test]
fn test_punishment_number() {
    assert_eq!(Solution::punishment_number(10), 182);
}

#[test]
fn test_punishment_number2() {
    assert_eq!(Solution::punishment_number(37), 1478);
}
