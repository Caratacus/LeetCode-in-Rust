// Tests for Problem 0802: Find Eventual Safe States
// Java reference: src/test/java/g0701_0800/s0802_find_eventual_safe_states/SolutionTest.java

use leetcode_in_rust::s0802::find_eventual_safe_states::Solution;

#[test]
fn test_eventual_safe_nodes() {
    let mut result = Solution::eventual_safe_nodes(vec![
        vec![1, 2],
        vec![2, 3],
        vec![5],
        vec![0],
        vec![5],
        vec![],
        vec![],
    ]);
    result.sort();
    assert_eq!(result, vec![2, 4, 5, 6]);
}

#[test]
fn test_eventual_safe_nodes2() {
    let mut result = Solution::eventual_safe_nodes(vec![
        vec![1, 2, 3, 4],
        vec![1, 2],
        vec![3, 4],
        vec![0, 4],
        vec![],
    ]);
    result.sort();
    assert_eq!(result, vec![4]);
}
