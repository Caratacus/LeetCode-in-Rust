// Tests for Problem 1954: Minimum Garden Perimeter to Collect Enough Apples
// Java reference: src/test/java/g1901_2000/s1954_minimum_garden_perimeter_to_collect_enough_apples/SolutionTest.java

use leetcode_in_rust::s1954::minimum_garden_perimeter_to_collect_enough_apples::Solution;

#[test]
fn test_minimum_perimeter() {
    assert_eq!(Solution::minimum_perimeter(1), 8);
}

#[test]
fn test_minimum_perimeter2() {
    assert_eq!(Solution::minimum_perimeter(13), 16);
}

#[test]
fn test_minimum_perimeter3() {
    assert_eq!(Solution::minimum_perimeter(1000000000), 5040);
}
