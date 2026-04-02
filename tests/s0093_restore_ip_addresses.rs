// Tests for Problem 0093: Restore IP Addresses
// Java reference: src/test/java/g0001_0100/s0093_restore_ip_addresses/SolutionTest.java

use leetcode_in_rust::s0093::restore_ip_addresses::Solution;
use leetcode_in_rust::utils::array_utils::compare_unsorted;

#[test]
fn test_restore_ip_addresses() {
    let result = Solution::restore_ip_addresses("25525511135".to_string());
    let expected = vec!["255.255.11.135".to_string(), "255.255.111.35".to_string()];
    assert!(compare_unsorted(&result, &expected));
}

#[test]
fn test_restore_ip_addresses2() {
    let result = Solution::restore_ip_addresses("0000".to_string());
    let expected = vec!["0.0.0.0".to_string()];
    assert!(compare_unsorted(&result, &expected));
}

#[test]
fn test_restore_ip_addresses3() {
    let result = Solution::restore_ip_addresses("101023".to_string());
    let expected = vec![
        "1.0.10.23".to_string(), "1.0.102.3".to_string(),
        "10.1.0.23".to_string(), "10.10.2.3".to_string(),
        "101.0.2.3".to_string(),
    ];
    assert!(compare_unsorted(&result, &expected));
}
