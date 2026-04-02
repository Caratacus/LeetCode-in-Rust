// Tests for Problem 0347: Top K Frequent Elements
// Java reference: src/test/java/g0301_0400/s0347_top_k_frequent_elements/SolutionTest.java

use leetcode_in_rust::s0347::top_k_frequent_elements::Solution;

#[test]
fn test_top_k_frequent() {
    let mut result = Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
    result.sort();
    assert_eq!(result, vec![1, 2]);
}

#[test]
fn test_top_k_frequent2() {
    let result = Solution::top_k_frequent(vec![1], 1);
    assert_eq!(result, vec![1]);
}
