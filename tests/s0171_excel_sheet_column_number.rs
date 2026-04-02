// Tests for Problem 0171: Excel Sheet Column Number
// Java reference: src/test/java/g0121_0200/s0171_excel_sheet_column_number/SolutionTest.java

use leetcode_in_rust::s0171::excel_sheet_column_number::Solution;

#[test]
fn test_title_to_number() {
    assert_eq!(Solution::title_to_number(String::from("A")), 1);
}

#[test]
fn test_title_to_number2() {
    assert_eq!(Solution::title_to_number(String::from("AB")), 28);
}

#[test]
fn test_title_to_number3() {
    assert_eq!(Solution::title_to_number(String::from("ZY")), 701);
}
