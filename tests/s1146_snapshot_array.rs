// Tests for Problem 1146: Snapshot Array
// Java reference: src/test/java/g1101_1200/s1146_snapshot_array/SolutionTest.java

use leetcode_in_rust::s1146::snapshot_array::SnapshotArray;

#[test]
fn test_snapshot_array_test() {
    let mut snapshot_arr = SnapshotArray::new(3);
    snapshot_arr.set(0, 5);
    assert_eq!(snapshot_arr.snap(), 0);
    snapshot_arr.set(0, 6);
    assert_eq!(snapshot_arr.get(0, 0), 5);
}
