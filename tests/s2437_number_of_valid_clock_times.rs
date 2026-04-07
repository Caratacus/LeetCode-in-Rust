// Tests for Problem 2437: Number of Valid Clock Times
// Java reference: src/test/java/g2401_2500/s2437_number_of_valid_clock_times/SolutionTest.java

use leetcode_in_rust::s2437::number_of_valid_clock_times::Solution;

#[test]
fn test_count_time() {
    assert_eq!(Solution::count_time("?5:00".to_string()), 2);
}

#[test]
fn test_count_time2() {
    assert_eq!(Solution::count_time("0?:0?".to_string()), 100);
}

#[test]
fn test_count_time3() {
    assert_eq!(Solution::count_time("??:??".to_string()), 1440);
}
