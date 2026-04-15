// Problem 3380: maximum area rectangle with point constraints i
// #Medium #Array #Math #Sorting #Enumeration #Geometry

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn max_rectangle_area(points: Vec<Vec<i32>>) -> i32 {
        let set: HashSet<(i32, i32)> = points.iter().map(|p| (p[0], p[1])).collect();
        let mut max_area = -1;
        let n = points.len();
        for i in 0..n {
            for j in 1..n {
                let (x1, y1) = (points[i][0], points[i][1]);
                let (x2, y2) = (points[j][0], points[j][1]);
                if x1 == x2 || y1 == y2 {
                    continue;
                }
                if !set.contains(&(x1, y2)) || !set.contains(&(x2, y1)) {
                    continue;
                }
                if !Self::validate(&points, (x1, y1), (x2, y2)) {
                    continue;
                }
                max_area = max_area.max((y2 - y1).abs() * (x2 - x1).abs());
            }
        }
        max_area
    }

    fn validate(points: &Vec<Vec<i32>>, p1: (i32, i32), p2: (i32, i32)) -> bool {
        let top = p1.1.max(p2.1);
        let bot = p1.1.min(p2.1);
        let left = p1.0.min(p2.0);
        let right = p1.0.max(p2.0);
        for p in points {
            let (x, y) = (p[0], p[1]);
            if ((y == top || y == bot) && x > left && x < right)
                || ((x == left || x == right) && y > bot && y < top)
                || (x > left && x < right && y > bot && y < top)
            {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maxRectangleArea()
    //   assertThat(
    //   new Solution().maxRectangleArea(new int[][] {{1, 1}, {1, 3}, {3, 1}, {3, 3}}),
    //   equalTo(4));
    #[test]
    fn test_max_rectangle_area() {
        // TODO: 翻译 Java 测试
    }

    // Java: void maxRectangleArea2()
    //   assertThat(
    //   new Solution()
    //   .maxRectangleArea(new int[][] {{1, 1}, {1, 3}, {3, 1}, {3, 3}, {2, 2}}),
    //   equalTo(-1));
    #[test]
    fn test_max_rectangle_area2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void maxRectangleArea3()
    //   assertThat(
    //   new Solution()
    //   .maxRectangleArea(
    //   new int[][] {{1, 1}, {1, 3}, {3, 1}, {3, 3}, {1, 2}, {3, 2}}),
    //   equalTo(2));
    #[test]
    fn test_max_rectangle_area3() {
        // TODO: 翻译 Java 测试
    }
}
