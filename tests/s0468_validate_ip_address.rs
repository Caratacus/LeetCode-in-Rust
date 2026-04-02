// Tests for Problem 0468: Validate IP Address
// Java reference: src/test/java/g0401_0500/s0468_validate_ip_address/SolutionTest.java

use leetcode_in_rust::s0468::validate_ip_address::Solution;

#[test]
fn test_valid_ip_address() {
    assert_eq!(Solution::valid_ip_address("172.16.254.1".to_string()), "IPv4");
}

#[test]
fn test_valid_ip_address2() {
    assert_eq!(
        Solution::valid_ip_address("2001:0db8:85a3:0:0:8A2E:0370:7334".to_string()),
        "IPv6"
    );
}

#[test]
fn test_valid_ip_address3() {
    assert_eq!(Solution::valid_ip_address("256.256.256.256".to_string()), "Neither");
}
