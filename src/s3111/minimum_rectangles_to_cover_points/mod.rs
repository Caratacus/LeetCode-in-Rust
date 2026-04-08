// Problem 3111: Minimum Rectangles to Cover Points
// #Medium #Array #Sorting #Greedy #2024_04_27_Time_4_ms_(99.55%)_Space_97.4_MB_(47.05%)

pub struct Solution;

impl Solution {
    pub fn min_rectangles_to_cover_points(mut points: Vec<Vec<i32>>, w: i32) -> i32 {
        points.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut res = 0;
        let mut last = -1;
        for a in &points {
            if a[0] > last {
                res += 1;
                last = a[0] + w;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minRectanglesToCoverPoints()
    //   assertThat(
    //   new Solution()
    //   .minRectanglesToCoverPoints(
    //   new int[][] {{2, 1}, {1, 0}, {1, 4}, {1, 8}, {3, 5}, {4, 6}}, 1),
    //   equalTo(2));
    #[test]
    fn test_min_rectangles_to_cover_points() {
        assert_eq!(
            Solution::min_rectangles_to_cover_points(
                vec![vec![2, 1], vec![1, 0], vec![1, 4], vec![1, 8], vec![3, 5], vec![4, 6]],
                1
            ),
            2
        );
    }

    // Java: void minRectanglesToCoverPoints2()
    //   assertThat(
    //   new Solution()
    //   .minRectanglesToCoverPoints(
    //   new int[][] {
    //   {0, 0}, {1, 1}, {2, 2}, {3, 3}, {4, 4}, {5, 5}, {6, 6}, {7, 7}, {8, 8}, {9, 9}
    //   }, 2),
    //   equalTo(5));
    #[test]
    fn test_min_rectangles_to_cover_points2() {
        assert_eq!(
            Solution::min_rectangles_to_cover_points(
                vec![
                    vec![0, 0],
                    vec![1, 1],
                    vec![2, 2],
                    vec![3, 3],
                    vec![4, 4],
                    vec![5, 5],
                    vec![6, 6],
                    vec![7, 7],
                    vec![8, 8],
                    vec![9, 9]
                ],
                2
            ),
            5
        );
    }

    // Java: void minRectanglesToCoverPoints3()
    //   assertThat(
    //   new Solution().minRectanglesToCoverPoints(new int[][] {{2, 3}, {1, 2}}, 0),
    //   equalTo(2));
    #[test]
    fn test_min_rectangles_to_cover_points3() {
        assert_eq!(
            Solution::min_rectangles_to_cover_points(vec![vec![2, 3], vec![1, 2]], 0),
            2
        );
    }
}
