// Tests for Problem 1743: Restore the Array From Adjacent Pairs
// Java reference: src/test/java/g1701_1800/s1743_restore_the_array_from_adjacent_pairs/SolutionTest.java

use leetcode_in_rust::s1743::restore_the_array_from_adjacent_pairs::Solution;

#[test]
fn test_restore_array() {
    assert_eq!(
        Solution::restore_array(vec![vec![2, 1], vec![3, 4], vec![3, 2]]),
        vec![1, 2, 3, 4]
    );
}

#[test]
fn test_restore_array2() {
    assert_eq!(
        Solution::restore_array(vec![vec![4, -2], vec![1, 4], vec![-3, 1]]),
        vec![-2, 4, 1, -3]
    );
}

#[test]
fn test_restore_array3() {
    assert_eq!(
        Solution::restore_array(vec![vec![100000, -100000]]),
        vec![100000, -100000]
    );
}
