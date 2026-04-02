// Tests for Problem 1093: Statistics from a Large Sample
// Java reference: src/test/java/g1001_1100/s1093_statistics_from_a_large_sample/SolutionTest.java

use leetcode_in_rust::s1093::statistics_from_a_large_sample::Solution;

#[test]
fn test_sample_stats() {
    let mut count = [0i32; 256];
    count[1] = 1;
    count[2] = 3;
    count[3] = 4;
    assert_eq!(
        Solution::sample_stats(count.to_vec()),
        vec![1.0, 3.0, 2.375, 2.5, 3.0]
    );
}
