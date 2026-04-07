// Tests for Problem 2136: Earliest Possible Day of Full Bloom
// Java reference: src/test/java/g2101_2200/s2136_earliest_possible_day_of_full_bloom/SolutionTest.java

use leetcode_in_rust::s2136::earliest_possible_day_of_full_bloom::Solution;

#[test]
fn test_earliest_full_bloom() {
    assert_eq!(
        Solution::earliest_full_bloom(vec![1, 4, 3], vec![2, 3, 1]),
        9
    );
}

#[test]
fn test_earliest_full_bloom2() {
    assert_eq!(
        Solution::earliest_full_bloom(vec![1, 2, 3, 2], vec![2, 1, 2, 1]),
        9
    );
}

#[test]
fn test_earliest_full_bloom3() {
    assert_eq!(Solution::earliest_full_bloom(vec![1], vec![1]), 2);
}
