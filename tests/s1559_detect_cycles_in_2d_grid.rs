// Tests for Problem 1559: Detect Cycles in 2D Grid
// Java reference: src/test/java/g1501_1600/s1559_detect_cycles_in_2d_grid/SolutionTest.java

use leetcode_in_rust::s1559::detect_cycles_in_2d_grid::Solution;

#[test]
fn test_contains_cycle() {
    assert_eq!(
        Solution::contains_cycle(vec![
            vec!['a', 'a', 'a', 'a'],
            vec!['a', 'b', 'b', 'a'],
            vec!['a', 'b', 'b', 'a'],
            vec!['a', 'a', 'a', 'a']
        ]),
        true
    );
}

#[test]
fn test_contains_cycle2() {
    assert_eq!(
        Solution::contains_cycle(vec![
            vec!['c', 'c', 'c', 'a'],
            vec!['c', 'd', 'c', 'c'],
            vec!['c', 'c', 'e', 'c'],
            vec!['f', 'c', 'c', 'c']
        ]),
        true
    );
}

#[test]
fn test_contains_cycle3() {
    assert_eq!(
        Solution::contains_cycle(vec![
            vec!['a', 'b', 'b'],
            vec!['b', 'z', 'b'],
            vec!['b', 'b', 'a']
        ]),
        false
    );
}
