// Tests for Problem 2332: The Latest Time to Catch a Bus
// Java reference: src/test/java/g2301_2400/s2332_the_latest_time_to_catch_a_bus/SolutionTest.java

use leetcode_in_rust::s2332::the_latest_time_to_catch_a_bus::Solution;

#[test]
fn test_latest_time_catch_the_bus() {
    assert_eq!(
        Solution::latest_time_catch_the_bus(vec![10, 20], vec![2, 17, 18, 19], 2),
        16
    );
}

#[test]
fn test_latest_time_catch_the_bus2() {
    assert_eq!(
        Solution::latest_time_catch_the_bus(
            vec![20, 30, 10],
            vec![19, 13, 26, 4, 25, 11, 21],
            2
        ),
        20
    );
}

#[test]
fn test_latest_time_catch_the_bus3() {
    assert_eq!(
        Solution::latest_time_catch_the_bus(vec![3], vec![2, 4], 2),
        3
    );
}
