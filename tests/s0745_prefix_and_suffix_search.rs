// Tests for Problem 0745: Prefix and Suffix Search
// Java reference: src/test/java/g0701_0800/s0745_prefix_and_suffix_search/WordFilterTest.java

use leetcode_in_rust::s0745::prefix_and_suffix_search::WordFilter;

#[test]
fn test_word_filter() {
    let mut word_filter = WordFilter::new(vec!["apple".to_string()]);
    assert_eq!(word_filter.f("a".to_string(), "e".to_string()), 0);
}
