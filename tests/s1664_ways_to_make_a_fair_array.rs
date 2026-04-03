// Tests for Problem 1664: Ways to Make a Fair Array
// Java reference: src/test/java/g1601_1700/s1664_ways_to_make_a_fair_array/SolutionTest.java

use leetcode_in_rust::s1664::ways_to_make_a_fair_array::Solution;

#[test]
fn test_ways_to_make_fair() {
    assert_eq!(Solution::ways_to_make_fair(vec![2, 1, 6, 4]), 1);
}

#[test]
fn test_ways_to_make_fair2() {
    assert_eq!(Solution::ways_to_make_fair(vec![1, 1, 1]), 3);
}

#[test]
fn test_ways_to_make_fair3() {
    assert_eq!(Solution::ways_to_make_fair(vec![1, 2, 3]), 0);
}
