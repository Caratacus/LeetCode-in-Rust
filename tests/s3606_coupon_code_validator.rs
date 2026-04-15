// Tests for Problem 3606: Coupon Code Validator
// Java reference: src/test/java/g3601_3700/s3606_coupon_code_validator/SolutionTest.java
use leetcode_in_rust::s3606::coupon_code_validator::Solution;
#[test]
fn test_validate_coupons() {
    assert_eq!(
        Solution::validate_coupons(
            vec!["SAVE20".to_string(), "".to_string(), "PHARMA5".to_string(), "SAVE@20".to_string()],
            vec!["restaurant".to_string(), "grocery".to_string(), "pharmacy".to_string(), "restaurant".to_string()],
            vec![true, true, true, true]
        ),
        vec!["PHARMA5".to_string(), "SAVE20".to_string()]
    );
}
#[test]
fn test_validate_coupons2() {
    assert_eq!(
        Solution::validate_coupons(
            vec!["GROCERY15".to_string(), "ELECTRONICS_50".to_string(), "DISCOUNT10".to_string()],
            vec!["grocery".to_string(), "electronics".to_string(), "invalid".to_string()],
            vec![false, true, true]
        ),
        vec!["ELECTRONICS_50".to_string()]
    );
}
