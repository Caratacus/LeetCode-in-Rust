// Tests for Problem 0449: Serialize and Deserialize BST
// Java reference: src/test/java/g0401_0500/s0449_serialize_and_deserialize_bst/SolutionTest.java

use leetcode_in_rust::s0449::serialize_and_deserialize_bst::Codec;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_serialize_deserialize() {
    let mut codec = Codec {};
    let root = tree_from_vec(vec![Some(3), Some(1), Some(4), None, Some(2)]);
    let serialized = codec.serialize(root);
    let deserialized = codec.deserialize(serialized);
    // Result should be a valid tree
    assert!(deserialized.is_some());
}
