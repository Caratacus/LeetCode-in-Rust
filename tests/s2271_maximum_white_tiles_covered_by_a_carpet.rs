// Tests for Problem 2271: Maximum White Tiles Covered by a Carpet
// Java reference: src/test/java/g2201_2300/s2271_maximum_white_tiles_covered_by_a_carpet/SolutionTest.java

use leetcode_in_rust::s2271::maximum_white_tiles_covered_by_a_carpet::Solution;

#[test]
fn test_maximum_white_tiles() {
    assert_eq!(
        Solution::maximum_white_tiles(
            vec![vec![1, 5], vec![10, 11], vec![12, 18], vec![20, 25], vec![30, 32]],
            10
        ),
        9
    );
}

#[test]
fn test_maximum_white_tiles2() {
    assert_eq!(
        Solution::maximum_white_tiles(vec![vec![10, 11], vec![1, 1]], 2),
        2
    );
}
