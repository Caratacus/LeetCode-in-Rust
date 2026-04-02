// Tests for Problem 0929: Unique Email Addresses
// Java reference: src/test/java/g0901_1000/s0929_unique_email_addresses/SolutionTest.java

use leetcode_in_rust::s0929::unique_email_addresses::Solution;

#[test]
fn test_num_unique_emails() {
    assert_eq!(
        Solution::num_unique_emails(vec![
            "test.email+alex@leetcode.com".to_string(),
            "test.e.mail+bob.cathy@leetcode.com".to_string(),
            "testemail+david@lee.tcode.com".to_string()
        ]),
        2
    );
}

#[test]
fn test_num_unique_emails2() {
    assert_eq!(
        Solution::num_unique_emails(vec![
            "a@leetcode.com".to_string(),
            "b@leetcode.com".to_string(),
            "c@leetcode.com".to_string()
        ]),
        3
    );
}
