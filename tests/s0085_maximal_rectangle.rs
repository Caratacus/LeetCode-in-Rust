// Tests for Problem 0085: Maximal Rectangle
// Java reference: src/test/java/g0001_0100/s0085_maximal_rectangle/SolutionTest.java

use leetcode_in_rust::s0085::maximal_rectangle::Solution;

#[test]
fn test_maximal_rectangle() {
    let matrix = vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0'],
    ];
    assert_eq!(Solution::maximal_rectangle(matrix), 6);
}

#[test]
fn test_maximal_rectangle2() {
    let matrix = vec![vec!['0']];
    assert_eq!(Solution::maximal_rectangle(matrix), 0);
}

#[test]
fn test_maximal_rectangle3() {
    let matrix = vec![vec!['1']];
    assert_eq!(Solution::maximal_rectangle(matrix), 1);
}
