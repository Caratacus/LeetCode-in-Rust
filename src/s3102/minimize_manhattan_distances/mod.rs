// Problem 3102: Minimize Manhattan Distances
// #Hard #Array #Math #2024_04_20_Time_3_ms_(99.73%)_Space_82.4_MB_(44.95%)

pub struct Solution;

impl Solution {
    fn manhattan(points: &[Vec<i32>], i: usize, j: usize) -> i32 {
        (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs()
    }

    fn max_manhattan_distance(points: &[Vec<i32>], remove: i32) -> (usize, usize) {
        let n = points.len();
        let mut max_sum = i32::MIN;
        let mut min_sum = i32::MAX;
        let mut max_diff = i32::MIN;
        let mut min_diff = i32::MAX;
        let mut max_sum_index: usize = 0;
        let mut min_sum_index: usize = 0;
        let mut max_diff_index: usize = 0;
        let mut min_diff_index: usize = 0;

        for i in 0..n {
            if i as i32 != remove {
                let sum = points[i][0] + points[i][1];
                let diff = points[i][0] - points[i][1];
                if sum > max_sum {
                    max_sum_index = i;
                    max_sum = sum;
                }
                if sum < min_sum {
                    min_sum_index = i;
                    min_sum = sum;
                }
                if diff > max_diff {
                    max_diff_index = i;
                    max_diff = diff;
                }
                if diff < min_diff {
                    min_diff_index = i;
                    min_diff = diff;
                }
            }
        }

        if max_sum - min_sum >= max_diff - min_diff {
            (max_sum_index, min_sum_index)
        } else {
            (max_diff_index, min_diff_index)
        }
    }

    pub fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
        let m = Self::max_manhattan_distance(&points, -1);
        let m1 = Self::max_manhattan_distance(&points, m.0 as i32);
        let m2 = Self::max_manhattan_distance(&points, m.1 as i32);
        std::cmp::min(
            Self::manhattan(&points, m1.0, m1.1),
            Self::manhattan(&points, m2.0, m2.1),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minimumDistance()
    #[test]
    fn test_minimum_distance() {
        assert_eq!(
            Solution::minimum_distance(vec![vec![3, 10], vec![5, 15], vec![10, 2], vec![4, 4]]),
            12
        );
    }

    // Java: void minimumDistance2()
    #[test]
    fn test_minimum_distance2() {
        assert_eq!(
            Solution::minimum_distance(vec![vec![1, 1], vec![1, 1], vec![1, 1]]),
            0
        );
    }
}
