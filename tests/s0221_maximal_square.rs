// Tests for Problem 0221: Maximal Square
// Java reference: src/test/java/g0201_0300/s0221_maximal_square/SolutionTest.java

use leetcode_in_rust::s0221::maximal_square::Solution;

#[test]
fn test_maximal_square() {
    let matrix = vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0'],
    ];
    assert_eq!(Solution::maximal_square(matrix), 4);
}

#[test]
fn test_maximal_square2() {
    let matrix = vec![vec!['0', '1'], vec!['1', '0']];
    assert_eq!(Solution::maximal_square(matrix), 1);
}

#[test]
fn test_maximal_square3() {
    let matrix = vec![vec!['0']];
    assert_eq!(Solution::maximal_square(matrix), 0);
}
