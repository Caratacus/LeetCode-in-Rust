// Tests for Problem 2169: Count Operations to Obtain Zero
// Java reference: src/test/java/g2101_2200/s2169_count_operations_to_obtain_zero/SolutionTest.java

use leetcode_in_rust::s2169::count_operations_to_obtain_zero::Solution;

#[test]
fn test_count_operations() {
    assert_eq!(Solution::count_operations(2, 3), 3);
}

#[test]
fn test_count_operations2() {
    assert_eq!(Solution::count_operations(10, 10), 1);
}
