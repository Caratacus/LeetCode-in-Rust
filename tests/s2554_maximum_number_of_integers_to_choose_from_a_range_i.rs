// Tests for Problem 2554: Maximum Number of Integers to Choose From a Range I
// Java reference: src/test/java/g2501_2600/s2554_maximum_number_of_integers_to_choose_from_a_range_i/SolutionTest.java

use leetcode_in_rust::s2554::maximum_number_of_integers_to_choose_from_a_range_i::Solution;

#[test]
fn test_max_count() {
    assert_eq!(Solution::max_count(vec![1, 6, 5], 5, 6), 2);
}

#[test]
fn test_max_count2() {
    assert_eq!(Solution::max_count(vec![1, 2, 3, 4, 5, 6, 7], 8, 1), 0);
}

#[test]
fn test_max_count3() {
    assert_eq!(Solution::max_count(vec![11], 7, 50), 7);
}
