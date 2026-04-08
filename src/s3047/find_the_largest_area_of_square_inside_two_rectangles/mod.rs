// Problem 3047: Find the Largest Area of Square Inside Two Rectangles
// #Medium #Array #Math #Geometry
// #Big_O_Time_O(n^2)_Space_O(1)

pub struct Solution;

impl Solution {
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        let n = bottom_left.len();
        let mut max_area: i64 = 0;

        for i in 0..n {
            let ax = bottom_left[i][0];
            let ay = bottom_left[i][1];
            let bx = top_right[i][0];
            let by = top_right[i][1];

            for j in (i + 1)..n {
                let cx = bottom_left[j][0];
                let cy = bottom_left[j][1];
                let dx = top_right[j][0];
                let dy = top_right[j][1];

                let x1 = ax.max(cx);
                let y1 = ay.max(cy);
                let x2 = bx.min(dx);
                let y2 = by.min(dy);

                let min_side = (x2 - x1).min(y2 - y1);
                let area = (min_side.max(0) as i64).pow(2);
                max_area = max_area.max(area);
            }
        }
        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_square_area() {
        assert_eq!(
            Solution::largest_square_area(
                vec![vec![1, 1], vec![2, 2], vec![3, 1]],
                vec![vec![3, 3], vec![4, 4], vec![6, 6]]
            ),
            1
        );
    }

    #[test]
    fn test_largest_square_area2() {
        assert_eq!(
            Solution::largest_square_area(
                vec![vec![1, 1], vec![2, 2], vec![1, 2]],
                vec![vec![3, 3], vec![4, 4], vec![3, 4]]
            ),
            1
        );
    }

    #[test]
    fn test_largest_square_area3() {
        assert_eq!(
            Solution::largest_square_area(
                vec![vec![1, 1], vec![3, 3], vec![3, 1]],
                vec![vec![2, 2], vec![4, 4], vec![4, 2]]
            ),
            0
        );
    }
}
