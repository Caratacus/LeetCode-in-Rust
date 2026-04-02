// Tests for Problem 0933: Number of Recent Calls
// Java reference: src/test/java/g0901_1000/s0933_number_of_recent_calls/SolutionTest.java

use leetcode_in_rust::s0933::number_of_recent_calls::RecentCounter;

#[test]
fn test_recent_counter() {
    let mut recent_counter = RecentCounter::new();
    assert_eq!(recent_counter.ping(1), 1);
    assert_eq!(recent_counter.ping(100), 2);
    assert_eq!(recent_counter.ping(3001), 3);
    assert_eq!(recent_counter.ping(3002), 3);
}
