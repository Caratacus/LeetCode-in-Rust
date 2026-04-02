// Tests for Problem 0303: Range Sum Query - Immutable
// Java reference: src/test/java/g0301_0400/s0303_range_sum_query_immutable/NumArrayTest.java

use leetcode_in_rust::s0303::range_sum_query_immutable::NumArray;

#[test]
fn test_num_array() {
    let mut num_array = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
    assert_eq!(num_array.sum_range(0, 2), 1);
    assert_eq!(num_array.sum_range(2, 5), -1);
    assert_eq!(num_array.sum_range(0, 5), -3);
}
