// Tests for Problem 2325: Decode the Message
// Java reference: src/test/java/g2301_2400/s2325_decode_the_message/SolutionTest.java

use leetcode_in_rust::s2325::decode_the_message::Solution;

#[test]
fn test_decode_message() {
    assert_eq!(
        Solution::decode_message(
            String::from("the quick brown fox jumps over the lazy dog"),
            String::from("vkbs bs t suepuv")
        ),
        String::from("this is a secret")
    );
}

#[test]
fn test_decode_message2() {
    assert_eq!(
        Solution::decode_message(
            String::from("eljuxhpwnyrdgtqkviszcfmabo"),
            String::from("zwx hnfx lqantp mnoeius ycgk vcnjrdb")
        ),
        String::from("the five boxing wizards jump quickly")
    );
}
