// Tests for Problem 1711: Count Good Meals
// Java reference: src/test/java/g1701_1800/s1711_count_good_meals/SolutionTest.java

use leetcode_in_rust::s1711::count_good_meals::Solution;

#[test]
fn test_count_pairs() {
    assert_eq!(Solution::count_pairs(vec![1, 3, 5, 7, 9]), 4);
}

#[test]
fn test_count_pairs2() {
    assert_eq!(Solution::count_pairs(vec![1, 1, 1, 3, 3, 3, 7]), 15);
}
