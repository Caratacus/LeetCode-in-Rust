// Tests for Problem 2579: Count Total Number of Colored Cells
// Java reference: src/test/java/g2501_2600/s2579_count_total_number_of_colored_cells/SolutionTest.java

use leetcode_in_rust::s2579::count_total_number_of_colored_cells::Solution;

#[test]
fn test_colored_cells() {
    assert_eq!(Solution::colored_cells(1), 1);
}

#[test]
fn test_colored_cells2() {
    assert_eq!(Solution::colored_cells(2), 5);
}
