// Tests for Problem 0307: Range Sum Query - Mutable
// Java reference: src/test/java/g0301_0400/s0307_range_sum_query_mutable/NumArrayTest.java

use leetcode_in_rust::s0307::range_sum_query_mutable::NumArray;

#[test]
fn test_num_array() {
    let mut num_array = NumArray::new(vec![1, 3, 5]);
    assert_eq!(num_array.sum_range(0, 2), 9);
    num_array.update(1, 2);
    assert_eq!(num_array.sum_range(0, 2), 8);
}
