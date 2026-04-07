// Tests for Problem 2315: Count Asterisks
// Java reference: src/test/java/g2301_2400/s2315_count_asterisks/SolutionTest.java

use leetcode_in_rust::s2315::count_asterisks::Solution;

#[test]
fn test_count_asterisks() {
    assert_eq!(Solution::count_asterisks(String::from("l|*e*et|c**o|*de|")), 2);
}

#[test]
fn test_count_asterisks2() {
    assert_eq!(Solution::count_asterisks(String::from("iamprogrammer")), 0);
}

#[test]
fn test_count_asterisks3() {
    assert_eq!(
        Solution::count_asterisks(String::from("yo|uar|e**|b|e***au|tifu|l")),
        5
    );
}
