// Tests for Problem 1105: Filling Bookcase Shelves
// Java reference: src/test/java/g1101_1200/s1105_filling_bookcase_shelves/SolutionTest.java

use leetcode_in_rust::s1105::filling_bookcase_shelves::Solution;

#[test]
fn test_min_height_shelves() {
    assert_eq!(
        Solution::min_height_shelves(
            vec![vec![1, 1], vec![2, 3], vec![2, 3], vec![1, 1], vec![1, 1], vec![1, 1], vec![1, 2]],
            4
        ),
        6
    );
}

#[test]
fn test_min_height_shelves2() {
    assert_eq!(
        Solution::min_height_shelves(vec![vec![1, 3], vec![2, 4], vec![3, 2]], 66),
        4
    );
}
