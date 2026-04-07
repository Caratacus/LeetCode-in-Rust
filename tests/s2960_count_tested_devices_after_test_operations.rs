// Tests for Problem 2960: Count Tested Devices After Test Operations
// Java reference: src/test/java/g2901_3000/s2960_count_tested_devices_after_test_operations/SolutionTest.java

use leetcode_in_rust::s2960::count_tested_devices_after_test_operations::Solution;

#[test]
fn test_count_tested_devices() {
    assert_eq!(Solution::count_tested_devices(vec![1, 1, 2, 1, 3]), 3);
}

#[test]
fn test_count_tested_devices2() {
    assert_eq!(Solution::count_tested_devices(vec![0, 1, 2]), 2);
}
