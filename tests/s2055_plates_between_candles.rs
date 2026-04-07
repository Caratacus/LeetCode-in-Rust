// Tests for Problem 2055: Plates Between Candles
// Java reference: src/test/java/g2001_2100/s2055_plates_between_candles/SolutionTest.java

use leetcode_in_rust::s2055::plates_between_candles::Solution;

#[test]
fn test_plates_between_candles() {
    let queries = vec![vec![2, 5], vec![5, 9]];
    assert_eq!(
        Solution::plates_between_candles("**|**|***|".to_string(), queries),
        vec![2, 3]
    );
}

#[test]
fn test_plates_between_candles2() {
    let queries = vec![vec![1, 17], vec![4, 5], vec![14, 17], vec![5, 11], vec![15, 16]];
    assert_eq!(
        Solution::plates_between_candles("***|**|*****|**||**|*".to_string(), queries),
        vec![9, 0, 0, 0, 0]
    );
}
