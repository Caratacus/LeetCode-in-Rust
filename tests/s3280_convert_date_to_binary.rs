// Tests for Problem 3280: Convert Date to Binary
// Java reference: src/test/java/g3201_3300/s3280_convert_date_to_binary/SolutionTest.java

use leetcode_in_rust::s3280::convert_date_to_binary::Solution;

#[test]
fn test_convert_date_to_binary() {
    assert_eq!(
        Solution::convert_date_to_binary("2080-02-29".to_string()),
        "100000100000-10-11101"
    );
}

#[test]
fn test_convert_date_to_binary2() {
    assert_eq!(
        Solution::convert_date_to_binary("1900-01-01".to_string()),
        "11101101100-1-1"
    );
}
