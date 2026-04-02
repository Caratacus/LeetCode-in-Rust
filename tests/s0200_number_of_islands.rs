// Tests for Problem 0200: Number of Islands
// Java reference: src/test/java/g0181_0200/s0200_number_of_islands/SolutionTest.java

use leetcode_in_rust::s0200::number_of_islands::Solution;

#[test]
fn test_num_islands() {
    let grid = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];
    assert_eq!(Solution::num_islands(grid), 1);
}

#[test]
fn test_num_islands2() {
    let grid = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];
    assert_eq!(Solution::num_islands(grid), 3);
}
