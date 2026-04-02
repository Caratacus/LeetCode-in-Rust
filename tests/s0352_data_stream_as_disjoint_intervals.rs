// Tests for Problem 0352: Data Stream as Disjoint Intervals
// Java reference: src/test/java/g0301_0400/s0352_data_stream_as_disjoint_intervals/SolutionTest.java

use leetcode_in_rust::s0352::data_stream_as_disjoint_intervals::SummaryRanges;

#[test]
fn test_get_intervals() {
    let mut summary_ranges = SummaryRanges::new();
    summary_ranges.add_num(1);
    let result = summary_ranges.get_intervals();
    assert_eq!(result, vec![vec![1, 1]]);
}

#[test]
fn test_get_intervals2() {
    let mut summary_ranges = SummaryRanges::new();
    summary_ranges.add_num(1);
    summary_ranges.add_num(3);
    let mut result = summary_ranges.get_intervals();
    result.sort();
    assert!(result == vec![vec![1, 1], vec![3, 3]] || result == vec![vec![3, 3], vec![1, 1]]);
}

#[test]
fn test_get_intervals3() {
    let mut summary_ranges = SummaryRanges::new();
    summary_ranges.add_num(1);
    summary_ranges.add_num(3);
    summary_ranges.add_num(7);
    let mut result = summary_ranges.get_intervals();
    result.sort();
    assert!(result.contains(&vec![1, 1]) && result.contains(&vec![3, 3]) && result.contains(&vec![7, 7]));
}

#[test]
fn test_get_intervals4() {
    let mut summary_ranges = SummaryRanges::new();
    summary_ranges.add_num(1);
    summary_ranges.add_num(2);
    summary_ranges.add_num(3);
    summary_ranges.add_num(7);
    let mut result = summary_ranges.get_intervals();
    result.sort();
    assert!(result.contains(&vec![1, 3]) && result.contains(&vec![7, 7]));
}

#[test]
fn test_get_intervals5() {
    let mut summary_ranges = SummaryRanges::new();
    summary_ranges.add_num(1);
    summary_ranges.add_num(2);
    summary_ranges.add_num(3);
    summary_ranges.add_num(6);
    summary_ranges.add_num(7);
    let mut result = summary_ranges.get_intervals();
    result.sort();
    assert!(result.contains(&vec![1, 3]) && result.contains(&vec![6, 7]));
}
