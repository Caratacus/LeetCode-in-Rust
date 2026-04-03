// Tests for Problem 0482: License Key Formatting
// Java reference: src/test/java/g0401_0500/s0482_license_key_formatting/SolutionTest.java

use leetcode_in_rust::s0482::license_key_formatting::Solution;

#[test]
fn test_license_key_formatting() {
    assert_eq!(
        Solution::license_key_formatting("5F3Z-2e-9-w".to_string(), 4),
        "5F3Z-2E9W"
    );
}

#[test]
fn test_license_key_formatting2() {
    assert_eq!(
        Solution::license_key_formatting("2-5g-3-J".to_string(), 2),
        "2-5G-3J"
    );
}
