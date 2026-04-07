// Tests for Problem 2080: Range Frequency Queries
// Java reference: src/test/java/g2001_2100/s2080_range_frequency_queries/SolutionTest.java

use leetcode_in_rust::s2080::range_frequency_queries::RangeFreqQuery;

#[test]
fn test_range_freq_query() {
    let mut range_freq_query = RangeFreqQuery::new(vec![12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]);
    assert_eq!(range_freq_query.query(1, 2, 4), 1);
    assert_eq!(range_freq_query.query(1, 2, 33), 1);
    assert_eq!(range_freq_query.query(0, 11, 33), 2);
}
