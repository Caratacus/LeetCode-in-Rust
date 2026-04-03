// Tests for Problem 0932: Beautiful Array
// Java reference: src/test/java/g0901_1000/s0932_beautiful_array/SolutionTest.java

use leetcode_in_rust::s0932::beautiful_array::Solution;

fn is_beautiful(arr: &[i32]) -> bool {
    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            for k in (j + 1)..arr.len() {
                if arr[i] + arr[k] == 2 * arr[j] {
                    return false;
                }
            }
        }
    }
    true
}

#[test]
fn test_beautiful_array() {
    let result = Solution::beautiful_array(4);
    assert_eq!(result.len(), 4);
    assert!(is_beautiful(&result));
}

#[test]
fn test_beautiful_array2() {
    let result = Solution::beautiful_array(5);
    assert_eq!(result.len(), 5);
    assert!(is_beautiful(&result));
}
