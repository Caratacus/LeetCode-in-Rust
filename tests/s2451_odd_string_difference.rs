// Tests for Problem 2451: Odd String Difference
// Java reference: src/test/java/g2401_2500/s2451_odd_string_difference/SolutionTest.java

use leetcode_in_rust::s2451::odd_string_difference::Solution;

#[test]
fn test_odd_string() {
    assert_eq!(
        Solution::odd_string(vec!["adc".to_string(), "wzy".to_string(), "abc".to_string()]),
        "abc"
    );
}

#[test]
fn test_odd_string2() {
    assert_eq!(
        Solution::odd_string(vec![
            "aaa".to_string(),
            "bob".to_string(),
            "ccc".to_string(),
            "ddd".to_string()
        ]),
        "bob"
    );
}
