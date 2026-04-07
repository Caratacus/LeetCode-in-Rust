// Tests for Problem 2671: Frequency Tracker
// Java reference: src/test/java/g2601_2700/s2671_frequency_tracker/FrequencyTrackerTest.java

use leetcode_in_rust::s2671::frequency_tracker::FrequencyTracker;

#[test]
fn test_frequency_tracker() {
    let mut frequency_tracker = FrequencyTracker::new();
    frequency_tracker.add(3);
    frequency_tracker.add(3);
    assert_eq!(frequency_tracker.has_frequency(2), true);
}

#[test]
fn test_frequency_tracker2() {
    let mut frequency_tracker = FrequencyTracker::new();
    frequency_tracker.add(1);
    frequency_tracker.delete_one(1);
    assert_eq!(frequency_tracker.has_frequency(1), false);
}
