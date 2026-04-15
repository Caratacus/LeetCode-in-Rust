// Tests for Problem 3529: Count Cells in Overlapping Horizontal and Vertical Substrings
// Java reference: src/test/java/g3501_3600/s3529_count_cells_in_overlapping_horizontal_and_vertical_substrings/SolutionTest.java

use leetcode_in_rust::s3529::count_cells_in_overlapping_horizontal_and_vertical_substrings::Solution;

#[test]
fn test_count_cells() {
    assert_eq!(
        Solution::count_cells(
            vec![vec!['a', 'a', 'c', 'c'], vec!['b', 'b', 'b', 'c'], vec!['a', 'a', 'b', 'a'], vec!['c', 'a', 'a', 'c'], vec!['a', 'a', 'c', 'c']],
            "abaca".to_string()
        ),
        1
    );
}

#[test]
fn test_count_cells2() {
    assert_eq!(
        Solution::count_cells(
            vec![vec!['c', 'a', 'a', 'a'], vec!['a', 'a', 'b', 'a'], vec!['b', 'b', 'a', 'a'], vec!['a', 'a', 'b', 'a']],
            "aba".to_string()
        ),
        4
    );
}

#[test]
fn test_count_cells3() {
    assert_eq!(Solution::count_cells(vec![vec!['a']], "a".to_string()), 1);
}
