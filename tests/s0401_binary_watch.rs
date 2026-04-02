// Tests for Problem 0401: Binary Watch
// Java reference: src/test/java/g0401_0500/s0401_binary_watch/SolutionTest.java

use leetcode_in_rust::s0401::binary_watch::Solution;

#[test]
fn test_read_binary_watch() {
    let result = Solution::read_binary_watch(1);
    let expected = vec![
        "0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00",
    ];
    assert_eq!(result.len(), expected.len());
    for time in &expected {
        assert!(result.contains(&time.to_string()));
    }
}

#[test]
fn test_read_binary_watch2() {
    let result = Solution::read_binary_watch(9);
    assert!(result.is_empty());
}
