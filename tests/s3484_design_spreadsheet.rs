// Tests for Problem 3484: Design Spreadsheet
// Java reference: src/test/java/g3401_3500/s3484_design_spreadsheet/SpreadsheetTest.java

use leetcode_in_rust::s3484::design_spreadsheet::Spreadsheet;

#[test]
fn test_spreadsheet() {
    let mut spreadsheet = Spreadsheet::new(3);
    assert_eq!(spreadsheet.get_value("=5+7".to_string()), 12);
    spreadsheet.set_cell("A1".to_string(), 10);
    assert_eq!(spreadsheet.get_value("=A1+6".to_string()), 16);
    spreadsheet.set_cell("B2".to_string(), 15);
    assert_eq!(spreadsheet.get_value("=A1+B2".to_string()), 25);
    spreadsheet.reset_cell("A1".to_string());
    assert_eq!(spreadsheet.get_value("=A1+B2".to_string()), 15);
}
