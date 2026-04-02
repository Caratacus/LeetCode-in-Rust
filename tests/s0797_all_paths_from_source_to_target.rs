// Tests for Problem 0797: All Paths From Source to Target
// Java reference: src/test/java/g0701_0800/s0797_all_paths_from_source_to_target/SolutionTest.java

use leetcode_in_rust::s0797::all_paths_from_source_to_target::Solution;

#[test]
fn test_all_paths_source_target() {
    let result = Solution::all_paths_source_target(vec![vec![1, 2], vec![3], vec![3], vec![]]);
    let mut result = result.clone();
    result.sort();
    let mut expected = vec![vec![0, 1, 3], vec![0, 2, 3]];
    expected.sort();
    assert_eq!(result, expected);
}

#[test]
fn test_all_paths_source_target2() {
    let result = Solution::all_paths_source_target(vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]]);
    let mut result = result.clone();
    result.sort();
    let mut expected = vec![
        vec![0, 4],
        vec![0, 3, 4],
        vec![0, 1, 3, 4],
        vec![0, 1, 2, 3, 4],
        vec![0, 1, 4],
    ];
    expected.sort();
    assert_eq!(result, expected);
}
