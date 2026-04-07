// Tests for Problem 1865: Finding Pairs With a Certain Sum
// Java reference: src/test/java/g1801_1900/s1865_finding_pairs_with_a_certain_sum/FindSumPairsTest.java

use leetcode_in_rust::s1865::finding_pairs_with_a_certain_sum::FindSumPairs;

#[test]
fn test_find_sum_pairs() {
    let mut obj = FindSumPairs::new(vec![1, 1, 2, 2, 2, 3], vec![1, 4, 5, 2, 5, 4]);
    assert_eq!(obj.count(7), 8);
    obj.add(3, 2);
    assert_eq!(obj.count(8), 2);
    obj.add(0, 1);
    obj.add(1, 1);
    assert_eq!(obj.count(7), 11);
}
