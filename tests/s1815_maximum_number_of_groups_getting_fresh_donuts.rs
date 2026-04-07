// Tests for Problem 1815: Maximum Number of Groups Getting Fresh Donuts
// Java reference: src/test/java/g1801_1900/s1815_maximum_number_of_groups_getting_fresh_donuts/SolutionTest.java

use leetcode_in_rust::s1815::maximum_number_of_groups_getting_fresh_donuts::Solution;

#[test]
fn test_max_happy_groups() {
    assert_eq!(Solution::max_happy_groups(3, vec![1, 2, 3, 4, 5, 6]), 4);
}

#[test]
fn test_max_happy_groups2() {
    assert_eq!(Solution::max_happy_groups(4, vec![1, 3, 2, 5, 2, 2, 1, 6]), 4);
}

#[test]
fn test_max_happy_groups5() {
    assert_eq!(Solution::max_happy_groups(1, vec![1, 2, 3]), 3);
}
