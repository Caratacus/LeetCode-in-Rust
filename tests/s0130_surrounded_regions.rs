// Tests for Problem 0130: Surrounded Regions
// Java reference: src/test/java/g0121_0200/s0130_surrounded_regions/SolutionTest.java

use leetcode_in_rust::s0130::surrounded_regions::Solution;

#[test]
fn test_solve() {
    let board = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    Solution::solve(board);
    // Function modifies in place but takes ownership
}
