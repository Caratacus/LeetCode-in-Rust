// Problem 3001: minimum moves to capture the queen
// #Medium #Array #Enumeration #2024_11_08_Time_0_ms_(100.00%)_Space_41_MB_(27.27%)

pub struct Solution;

impl Solution {
    pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
        if a == e || b == f {
            if a == e && a == c && (d - b) * (d - f) < 0 {
                return 2;
            }
            if b == f && b == d && (c - a) * (c - e) < 0 {
                return 2;
            }
            return 1;
        }
        if (c - e).abs() == (d - f).abs() {
            if (c - a).abs() == (d - b).abs() && (b - f) * (b - d) < 0 {
                return 2;
            }
            return 1;
        }
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_moves_to_capture_the_queen() {
        assert_eq!(Solution::min_moves_to_capture_the_queen(1, 1, 8, 8, 2, 3), 2);
    }

    #[test]
    fn test_min_moves_to_capture_the_queen2() {
        assert_eq!(Solution::min_moves_to_capture_the_queen(5, 3, 3, 4, 5, 2), 1);
    }

    #[test]
    fn test_min_moves_to_capture_the_queen3() {
        assert_eq!(Solution::min_moves_to_capture_the_queen(1, 1, 3, 1, 5, 1), 2);
    }

    #[test]
    fn test_min_moves_to_capture_the_queen4() {
        assert_eq!(Solution::min_moves_to_capture_the_queen(1, 1, 1, 3, 1, 5), 2);
    }

    #[test]
    fn test_min_moves_to_capture_the_queen5() {
        assert_eq!(Solution::min_moves_to_capture_the_queen(1, 1, 3, 3, 5, 5), 1);
    }

    #[test]
    fn test_min_moves_to_capture_the_queen6() {
        assert_eq!(Solution::min_moves_to_capture_the_queen(1, 1, 3, 1, 5, 3), 1);
    }

    #[test]
    fn test_min_moves_to_capture_the_queen7() {
        assert_eq!(Solution::min_moves_to_capture_the_queen(1, 1, 1, 3, 3, 5), 1);
    }

    #[test]
    fn test_min_moves_to_capture_the_queen8() {
        assert_eq!(Solution::min_moves_to_capture_the_queen(1, 1, 3, 3, 5, 1), 1);
    }

    #[test]
    fn test_min_moves_to_capture_the_queen9() {
        assert_eq!(Solution::min_moves_to_capture_the_queen(1, 1, 2, 3, 5, 5), 2);
    }

    #[test]
    fn test_min_moves_to_capture_the_queen10() {
        assert_eq!(Solution::min_moves_to_capture_the_queen(1, 2, 3, 2, 4, 2), 2);
    }

    #[test]
    fn test_min_moves_to_capture_the_queen11() {
        assert_eq!(Solution::min_moves_to_capture_the_queen(2, 3, 2, 4, 2, 6), 2);
    }

    #[test]
    fn test_min_moves_to_capture_the_queen12() {
        assert_eq!(Solution::min_moves_to_capture_the_queen(1, 1, 3, 3, 4, 4), 1);
    }

    #[test]
    fn test_min_moves_to_capture_the_queen13() {
        assert_eq!(Solution::min_moves_to_capture_the_queen(1, 2, 3, 2, 5, 2), 2);
    }

    #[test]
    fn test_min_moves_to_capture_the_queen14() {
        assert_eq!(Solution::min_moves_to_capture_the_queen(2, 1, 2, 3, 2, 5), 2);
    }

    #[test]
    fn test_min_moves_to_capture_the_queen15() {
        assert_eq!(Solution::min_moves_to_capture_the_queen(1, 1, 2, 2, 8, 8), 1);
    }
}
