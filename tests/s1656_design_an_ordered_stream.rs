// Tests for Problem 1656: Design an Ordered Stream
// Java reference: src/test/java/g1601_1700/s1656_design_an_ordered_stream/SolutionTest.java

use leetcode_in_rust::s1656::design_an_ordered_stream::OrderedStream;

#[test]
fn test_ordered_stream() {
    let mut os = OrderedStream::new(5);
    // Inserts (3, "ccccc"), returns [].
    assert_eq!(os.insert(3, "ccccc".to_string()), Vec::<String>::new());
    // Inserts (1, "aaaaa"), returns ["aaaaa"].
    assert_eq!(os.insert(1, "aaaaa".to_string()), vec!["aaaaa".to_string()]);
    // Inserts (2, "bbbbb"), returns ["bbbbb", "ccccc"].
    assert_eq!(os.insert(2, "bbbbb".to_string()), vec!["bbbbb".to_string(), "ccccc".to_string()]);
    // Inserts (5, "eeeee"), returns [].
    assert_eq!(os.insert(5, "eeeee".to_string()), Vec::<String>::new());
    // Inserts (4, "ddddd"), returns ["ddddd", "eeeee"].
    assert_eq!(os.insert(4, "ddddd".to_string()), vec!["ddddd".to_string(), "eeeee".to_string()]);
}
