// Tests for Problem 2363: Merge Similar Items
// Java reference: src/test/java/g2301_2400/s2363_merge_similar_items/SolutionTest.java

use leetcode_in_rust::s2363::merge_similar_items::Solution;

#[test]
fn test_merge_similar_items() {
    let arr1 = vec![vec![1, 1], vec![4, 5], vec![3, 8]];
    let arr2 = vec![vec![3, 1], vec![1, 5]];
    let result = Solution::merge_similar_items(arr1, arr2);
    let expected = vec![vec![1, 6], vec![3, 9], vec![4, 5]];
    assert_eq!(result, expected);
}

#[test]
fn test_merge_similar_items2() {
    let arr1 = vec![vec![1, 1], vec![3, 2], vec![2, 3]];
    let arr2 = vec![vec![2, 1], vec![3, 2], vec![1, 3]];
    let result = Solution::merge_similar_items(arr1, arr2);
    let expected = vec![vec![1, 4], vec![2, 4], vec![3, 4]];
    assert_eq!(result, expected);
}

#[test]
fn test_merge_similar_items3() {
    let arr1 = vec![vec![1, 3], vec![2, 2]];
    let arr2 = vec![vec![7, 1], vec![2, 2], vec![1, 4]];
    let result = Solution::merge_similar_items(arr1, arr2);
    let expected = vec![vec![1, 7], vec![2, 4], vec![7, 1]];
    assert_eq!(result, expected);
}
