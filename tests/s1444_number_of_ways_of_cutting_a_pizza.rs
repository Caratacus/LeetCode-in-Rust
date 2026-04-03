// Tests for Problem 1444: Number of Ways of Cutting a Pizza
// Java reference: src/test/java/g1401_1500/s1444_number_of_ways_of_cutting_a_pizza/SolutionTest.java

use leetcode_in_rust::s1444::number_of_ways_of_cutting_a_pizza::Solution;

#[test]
fn test_ways() {
    let pizza = vec!["A..".to_string(), "AAA".to_string(), "...".to_string()];
    assert_eq!(Solution::ways(pizza, 3), 3);
}

#[test]
fn test_ways2() {
    let pizza = vec!["A..".to_string(), "AA.".to_string(), "...".to_string()];
    assert_eq!(Solution::ways(pizza, 3), 1);
}

#[test]
fn test_ways3() {
    let pizza = vec!["A..".to_string(), "A..".to_string(), "...".to_string()];
    assert_eq!(Solution::ways(pizza, 1), 1);
}
