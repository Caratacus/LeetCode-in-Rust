// Tests for Problem 2276: Count Integers in Intervals
// Java reference: src/test/java/g2201_2300/s2276_count_integers_in_intervals/CountIntervalsTest.java

use leetcode_in_rust::s2276::count_integers_in_intervals::CountIntervals;

#[test]
fn test_count_intervals() {
    let mut count_intervals = CountIntervals::new();
    count_intervals.add(2, 3);
    count_intervals.add(7, 10);
    assert_eq!(count_intervals.count(), 6);
    count_intervals.add(5, 8);
    assert_eq!(count_intervals.count(), 8);
}
