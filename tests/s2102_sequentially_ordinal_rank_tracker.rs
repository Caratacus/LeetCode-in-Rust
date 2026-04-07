// Tests for Problem 2102: Sequentially Ordinal Rank Tracker
// Java reference: src/test/java/g2101_2200/s2102_sequentially_ordinal_rank_tracker/SolutionTest.java

use leetcode_in_rust::s2102::sequentially_ordinal_rank_tracker::SORTracker;

#[test]
fn test_sor_tracker() {
    let mut tracker = SORTracker::new();
    tracker.add("bradford".to_string(), 2);
    tracker.add("branford".to_string(), 3);
    assert_eq!(tracker.get(), "branford");
    tracker.add("alps".to_string(), 2);
    assert_eq!(tracker.get(), "alps");
    tracker.add("orland".to_string(), 2);
    assert_eq!(tracker.get(), "bradford");
    tracker.add("orlando".to_string(), 3);
    assert_eq!(tracker.get(), "bradford");
    tracker.add("alpine".to_string(), 2);
    assert_eq!(tracker.get(), "branford");
    assert_eq!(tracker.get(), "orland");
}
