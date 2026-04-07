// Tests for Problem 2766: Relocate Marbles
// Java reference: src/test/java/g2701_2800/s2766_relocate_marbles/SolutionTest.java

use leetcode_in_rust::s2766::relocate_marbles::Solution;

#[test]
fn test_relocate_marbles() {
    let mut result = Solution::relocate_marbles(
        vec![1, 6, 7, 8],
        vec![1, 7, 2],
        vec![2, 9, 5],
    );
    result.sort();
    assert_eq!(result, vec![5, 6, 8, 9]);
}

#[test]
fn test_relocate_marbles2() {
    let mut result = Solution::relocate_marbles(
        vec![1, 1, 3, 3],
        vec![1, 3],
        vec![2, 2],
    );
    result.sort();
    assert_eq!(result, vec![2]);
}
