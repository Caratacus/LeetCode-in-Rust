// Tests for Problem 1578: Minimum Time to Make Rope Colorful
// Java reference: src/test/java/g1501_1600/s1578_minimum_time_to_make_rope_colorful/SolutionTest.java

use leetcode_in_rust::s1578::minimum_time_to_make_rope_colorful::Solution;

#[test]
fn test_min_cost() {
    assert_eq!(Solution::min_cost("abaac".to_string(), vec![1, 2, 3, 4, 5]), 3);
}

#[test]
fn test_min_cost2() {
    assert_eq!(Solution::min_cost("abc".to_string(), vec![1, 2, 3]), 0);
}

#[test]
fn test_min_cost3() {
    assert_eq!(Solution::min_cost("aabaa".to_string(), vec![1, 2, 3, 4, 1]), 2);
}
