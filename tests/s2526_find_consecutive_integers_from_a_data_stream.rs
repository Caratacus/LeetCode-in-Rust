// Tests for Problem 2526: Find Consecutive Integers from a Data Stream
// Java reference: src/test/java/g2401_2500/s2526_find_consecutive_integers_from_a_data_stream/SolutionTest.java

use leetcode_in_rust::s2526::find_consecutive_integers_from_a_data_stream::DataStream;

#[test]
fn test_data_stream_test() {
    let mut data_stream = DataStream::new(4, 3);
    assert_eq!(data_stream.consec(4), false);
    assert_eq!(data_stream.consec(4), false);
    assert_eq!(data_stream.consec(4), true);
    assert_eq!(data_stream.consec(3), false);
}
