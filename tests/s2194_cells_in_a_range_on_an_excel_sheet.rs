// Tests for Problem 2194: Cells in a Range on an Excel Sheet
// Java reference: src/test/java/g2101_2200/s2194_cells_in_a_range_on_an_excel_sheet/SolutionTest.java

use leetcode_in_rust::s2194::cells_in_a_range_on_an_excel_sheet::Solution;

#[test]
fn test_cells_in_range() {
    assert_eq!(
        Solution::cells_in_range("K1:L2".to_string()),
        vec!["K1", "K2", "L1", "L2"]
    );
}

#[test]
fn test_cells_in_range2() {
    assert_eq!(
        Solution::cells_in_range("A1:F1".to_string()),
        vec!["A1", "B1", "C1", "D1", "E1", "F1"]
    );
}
