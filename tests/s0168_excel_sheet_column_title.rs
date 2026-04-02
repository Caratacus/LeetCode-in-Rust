// Tests for Problem 0168: Excel Sheet Column Title
// Java reference: src/test/java/g0121_0200/s0168_excel_sheet_column_title/SolutionTest.java

use leetcode_in_rust::s0168::excel_sheet_column_title::Solution;

#[test]
fn test_convert_to_title() {
    assert_eq!(Solution::convert_to_title(1), String::from("A"));
}

#[test]
fn test_convert_to_title2() {
    assert_eq!(Solution::convert_to_title(28), String::from("AB"));
}

#[test]
fn test_convert_to_title3() {
    assert_eq!(Solution::convert_to_title(701), String::from("ZY"));
}
