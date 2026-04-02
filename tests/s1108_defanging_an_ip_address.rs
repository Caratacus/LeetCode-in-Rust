// Tests for Problem 1108: Defanging an IP Address
// Java reference: src/test/java/g1101_1200/s1108_defanging_an_ip_address/SolutionTest.java

use leetcode_in_rust::s1108::defanging_an_ip_address::Solution;

#[test]
fn test_defang_i_paddr() {
    assert_eq!(Solution::defang_i_paddr("1.1.1.1".to_string()), "1[.]1[.]1[.]1");
}

#[test]
fn test_defang_i_paddr2() {
    assert_eq!(Solution::defang_i_paddr("255.100.50.0".to_string()), "255[.]100[.]50[.]0");
}
