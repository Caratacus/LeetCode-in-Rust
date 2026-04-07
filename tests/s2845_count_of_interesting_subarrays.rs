// Tests for Problem 2845: Count of Interesting Subarrays
// Java reference: src/test/java/g2801_2900/s2845_count_of_interesting_subarrays/SolutionTest.java

use leetcode_in_rust::s2845::count_of_interesting_subarrays::Solution;

#[test]
fn test_count_interesting_subarrays() {
    assert_eq!(Solution::count_interesting_subarrays(vec![3, 2, 4], 2, 1), 3);
}

#[test]
fn test_count_interesting_subarrays2() {
    assert_eq!(Solution::count_interesting_subarrays(vec![3, 1, 9, 6], 3, 0), 2);
}
