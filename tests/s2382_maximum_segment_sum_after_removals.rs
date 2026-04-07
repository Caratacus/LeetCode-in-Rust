// Tests for Problem 2382: Maximum Segment Sum After Removals
// Java reference: src/test/java/g2301_2400/s2382_maximum_segment_sum_after_removals/SolutionTest.java

use leetcode_in_rust::s2382::maximum_segment_sum_after_removals::Solution;

#[test]
fn test_maximum_segment_sum() {
    let nums = vec![1, 2, 5, 6, 1];
    let remove_queries = vec![0, 3, 2, 4, 1];
    let result = Solution::maximum_segment_sum(nums, remove_queries);
    let expected: Vec<i64> = vec![14, 7, 2, 2, 0];
    assert_eq!(result, expected);
}

#[test]
fn test_maximum_segment_sum2() {
    let nums = vec![3, 2, 11, 1];
    let remove_queries = vec![3, 2, 1, 0];
    let result = Solution::maximum_segment_sum(nums, remove_queries);
    let expected: Vec<i64> = vec![16, 5, 3, 0];
    assert_eq!(result, expected);
}
