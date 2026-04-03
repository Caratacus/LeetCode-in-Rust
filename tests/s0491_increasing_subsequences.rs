// Tests for Problem 0491: Increasing Subsequences
// Java reference: src/test/java/g0401_0500/s0491_increasing_subsequences/SolutionTest.java

use leetcode_in_rust::s0491::increasing_subsequences::Solution;

#[test]
fn test_find_subsequences() {
    let result = Solution::find_subsequences(vec![4, 6, 7, 7]);
    // Expected: [[4,6], [4,6,7], [4,6,7,7], [4,7], [4,7,7], [6,7], [6,7,7], [7,7]]
    assert!(result.contains(&vec![4, 6]));
    assert!(result.contains(&vec![4, 6, 7]));
    assert!(result.contains(&vec![4, 6, 7, 7]));
    assert!(result.contains(&vec![4, 7]));
    assert!(result.contains(&vec![4, 7, 7]));
    assert!(result.contains(&vec![6, 7]));
    assert!(result.contains(&vec![6, 7, 7]));
    assert!(result.contains(&vec![7, 7]));
    assert_eq!(result.len(), 8);
}

#[test]
fn test_find_subsequences2() {
    let result = Solution::find_subsequences(vec![4, 4, 3, 2, 1]);
    assert_eq!(result, vec![vec![4, 4]]);
}
