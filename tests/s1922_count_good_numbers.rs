// Tests for Problem 1922: Count Good Numbers
// Java reference: src/test/java/g1901_2000/s1922_count_good_numbers/SolutionTest.java

use leetcode_in_rust::s1922::count_good_numbers::Solution;

#[test]
fn test_count_good_numbers() {
    assert_eq!(Solution::count_good_numbers(1), 5);
}

#[test]
fn test_count_good_numbers2() {
    assert_eq!(Solution::count_good_numbers(4), 400);
}

#[test]
fn test_count_good_numbers3() {
    assert_eq!(Solution::count_good_numbers(50), 564908303);
}
