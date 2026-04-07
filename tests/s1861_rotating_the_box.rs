// Tests for Problem 1861: Rotating the Box
// Java reference: src/test/java/g1801_1900/s1861_rotating_the_box/SolutionTest.java

use leetcode_in_rust::s1861::rotating_the_box::Solution;

#[test]
fn test_rotate_the_box() {
    assert_eq!(
        Solution::rotate_the_box(vec![vec!['#', '.', '#']]),
        vec![vec!['.'], vec!['#'], vec!['#']]
    );
}

#[test]
fn test_rotate_the_box2() {
    assert_eq!(
        Solution::rotate_the_box(vec![
            vec!['#', '.', '*', '.'],
            vec!['#', '#', '*', '.']
        ]),
        vec![
            vec!['#', '.'],
            vec!['#', '#'],
            vec!['*', '*'],
            vec!['.', '.']
        ]
    );
}
