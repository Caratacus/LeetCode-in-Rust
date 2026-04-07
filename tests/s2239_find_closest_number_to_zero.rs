// Tests for Problem 2239: Find Closest Number to Zero
// Java reference: src/test/java/g2201_2300/s2239_find_closest_number_to_zero/SolutionTest.java

use leetcode_in_rust::s2239::find_closest_number_to_zero::Solution;

#[test]
fn test_find_closest_number() {
    assert_eq!(Solution::find_closest_number(vec![-4, -2, 1, 4, 8]), 1);
}

#[test]
fn test_find_closest_number2() {
    assert_eq!(Solution::find_closest_number(vec![2, -1, 1]), 1);
}
