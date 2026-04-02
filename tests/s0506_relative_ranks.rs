// Tests for Problem 0506: Relative Ranks
// Java reference: src/test/java/g0501_0600/s0506_relative_ranks/SolutionTest.java

use leetcode_in_rust::s0506::relative_ranks::Solution;

#[test]
fn test_find_relative_ranks() {
    assert_eq!(
        Solution::find_relative_ranks(vec![5, 4, 3, 2, 1]),
        vec!["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]
    );
}

#[test]
fn test_find_relative_ranks2() {
    assert_eq!(
        Solution::find_relative_ranks(vec![10, 3, 8, 9, 4]),
        vec!["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"]
    );
}
