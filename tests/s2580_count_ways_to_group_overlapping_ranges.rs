// Tests for Problem 2580: Count Ways to Group Overlapping Ranges
// Java reference: src/test/java/g2501_2600/s2580_count_ways_to_group_overlapping_ranges/SolutionTest.java

use leetcode_in_rust::s2580::count_ways_to_group_overlapping_ranges::Solution;

#[test]
fn test_count_ways() {
    assert_eq!(Solution::count_ways(vec![vec![6, 10], vec![5, 15]]), 2);
}

#[test]
fn test_count_ways2() {
    assert_eq!(
        Solution::count_ways(vec![vec![1, 3], vec![10, 20], vec![2, 5], vec![4, 8]]),
        4
    );
}
