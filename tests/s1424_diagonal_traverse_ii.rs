// Tests for Problem 1424: Diagonal Traverse II
// Java reference: src/test/java/g1401_1500/s1424_diagonal_traverse_ii/SolutionTest.java

use leetcode_in_rust::s1424::diagonal_traverse_ii::Solution;

#[test]
fn test_find_diagonal_order() {
    let input = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    assert_eq!(Solution::find_diagonal_order(input), vec![1, 4, 2, 7, 5, 3, 8, 6, 9]);
}

#[test]
fn test_find_diagonal_order2() {
    let input = vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7],
        vec![8],
        vec![9, 10, 11],
        vec![12, 13, 14, 15, 16],
    ];
    assert_eq!(Solution::find_diagonal_order(input), vec![1, 6, 2, 8, 7, 3, 9, 4, 12, 10, 5, 13, 11, 14, 15, 16]);
}
