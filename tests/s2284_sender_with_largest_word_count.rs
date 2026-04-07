// Tests for Problem 2284: Sender With Largest Word Count
// Java reference: src/test/java/g2201_2300/s2284_sender_with_largest_word_count/SolutionTest.java

use leetcode_in_rust::s2284::sender_with_largest_word_count::Solution;

#[test]
fn test_largest_word_count() {
    assert_eq!(
        Solution::largest_word_count(
            vec![
                String::from("Hello userTwooo"),
                String::from("Hi userThree"),
                String::from("Wonderful day Alice"),
                String::from("Nice day userThree")
            ],
            vec![
                String::from("Alice"),
                String::from("userTwo"),
                String::from("userThree"),
                String::from("Alice")
            ]
        ),
        String::from("Alice")
    );
}

#[test]
fn test_largest_word_count2() {
    assert_eq!(
        Solution::largest_word_count(
            vec![
                String::from("How is leetcode for everyone"),
                String::from("Leetcode is useful for practice")
            ],
            vec![String::from("Bob"), String::from("Charlie")]
        ),
        String::from("Charlie")
    );
}
