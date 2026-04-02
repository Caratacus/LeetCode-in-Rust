// Tests for Problem 0295: Find Median from Data Stream
// Java reference: src/test/java/g0201_0300/s0295_find_median_from_data_stream/MedianFinderTest.java

use leetcode_in_rust::s0295::find_median_from_data_stream::MedianFinder;

#[test]
fn test_median_finder() {
    let mut median_finder = MedianFinder::new();
    // arr = [1]
    median_finder.add_num(1);
    // arr = [1, 2]
    median_finder.add_num(2);
    // return 1.5 (i.e., (1 + 2) / 2)
    assert_eq!(median_finder.find_median(), 1.5);
    // arr = [1, 2, 3]
    median_finder.add_num(3);
    // return 2.0
    assert_eq!(median_finder.find_median(), 2.0);
}

#[test]
fn test_median_finder2() {
    let mut median_finder = MedianFinder::new();
    median_finder.add_num(1);
    median_finder.add_num(3);
    median_finder.add_num(-1);
    assert_eq!(median_finder.find_median(), 1.0);
}

#[test]
fn test_median_finder3() {
    let mut median_finder = MedianFinder::new();
    median_finder.add_num(42);
    assert_eq!(median_finder.find_median(), 42.0);
}

#[test]
fn test_median_finder4() {
    let mut median_finder = MedianFinder::new();
    median_finder.add_num(5);
    median_finder.add_num(5);
    median_finder.add_num(5);
    median_finder.add_num(5);
    assert_eq!(median_finder.find_median(), 5.0);
}

#[test]
fn test_median_finder5() {
    let mut median_finder = MedianFinder::new();
    median_finder.add_num(-5);
    median_finder.add_num(-10);
    median_finder.add_num(-3);
    assert_eq!(median_finder.find_median(), -5.0);
}
