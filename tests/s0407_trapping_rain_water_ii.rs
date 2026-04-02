// Tests for Problem 0407: Trapping Rain Water II
// Java reference: src/test/java/g0401_0500/s0407_trapping_rain_water_ii/SolutionTest.java

use leetcode_in_rust::s0407::trapping_rain_water_ii::Solution;

#[test]
fn test_trap_rain_water() {
    let height_map = vec![
        vec![1, 4, 3, 1, 3, 2],
        vec![3, 2, 1, 3, 2, 4],
        vec![2, 3, 3, 2, 3, 1],
    ];
    assert_eq!(Solution::trap_rain_water(height_map), 4);
}

#[test]
fn test_trap_rain_water2() {
    let height_map = vec![
        vec![3, 3, 3, 3, 3],
        vec![3, 2, 2, 2, 3],
        vec![3, 2, 1, 2, 3],
        vec![3, 2, 2, 2, 3],
        vec![3, 3, 3, 3, 3],
    ];
    assert_eq!(Solution::trap_rain_water(height_map), 10);
}
