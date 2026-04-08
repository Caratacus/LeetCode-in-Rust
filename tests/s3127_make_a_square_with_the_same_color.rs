// Tests for Problem 3127: Make a Square With the Same Color
// Java reference: src/test/java/g3101_3200/s3127_make_a_square_with_the_same_color/SolutionTest.java

use leetcode_in_rust::s3127::make_a_square_with_the_same_color::Solution;

#[test]
fn test_can_make_square() {
    assert_eq!(
        Solution::can_make_square(vec![
            vec!['B', 'W', 'B'],
            vec!['B', 'W', 'W'],
            vec!['B', 'W', 'B']
        ]),
        true
    );
}

#[test]
fn test_can_make_square2() {
    assert_eq!(
        Solution::can_make_square(vec![
            vec!['B', 'W', 'B'],
            vec!['W', 'B', 'W'],
            vec!['B', 'W', 'B']
        ]),
        false
    );
}

#[test]
fn test_can_make_square3() {
    assert_eq!(
        Solution::can_make_square(vec![
            vec!['B', 'W', 'B'],
            vec!['B', 'W', 'W'],
            vec!['B', 'W', 'W']
        ]),
        true
    );
}
