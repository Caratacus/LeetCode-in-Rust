// Tests for Problem 2860: Happy Students
// Java reference: src/test/java/g2801_2900/s2860_happy_students/SolutionTest.java

use leetcode_in_rust::s2860::happy_students::Solution;

#[test]
fn test_count_ways() {
    assert_eq!(Solution::count_ways(vec![1, 1]), 2);
}

#[test]
fn test_count_ways2() {
    assert_eq!(Solution::count_ways(vec![6, 0, 3, 3, 6, 7, 2, 7]), 3);
}
