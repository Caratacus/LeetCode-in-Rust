// Problem 3030: find the grid of region average
// #Medium #Array #Matrix #2024_03_01_Time_53_ms_(98.79%)_Space_88.4_MB_(35.20%)

pub struct Solution;

impl Solution {
    pub fn result_grid(image: Vec<Vec<i32>>, threshold: i32) -> Vec<Vec<i32>> {
        let n = image.len();
        let m = image[0].len();
        let mut intensity = vec![vec![0; m]; n];
        let mut count = vec![vec![0; m]; n];

        for i in 1..n - 1 {
            for j in 1..m - 1 {
                let mut region_possible = true;
                let mut region_sum = 0;
                let r0c0 = image[i - 1][j - 1];
                let r0c1 = image[i - 1][j];
                let r0c2 = image[i - 1][j + 1];
                let r1c0 = image[i][j - 1];
                let r1c1 = image[i][j];
                let r1c2 = image[i][j + 1];
                let r2c0 = image[i + 1][j - 1];
                let r2c1 = image[i + 1][j];
                let r2c2 = image[i + 1][j + 1];
                region_sum += r0c0 + r0c1 + r0c2 + r1c0 + r1c1 + r1c2 + r2c0 + r2c1 + r2c2;

                if (r0c0 - r0c1).abs() > threshold
                    || (r0c0 - r1c0).abs() > threshold
                    || (r0c1 - r0c0).abs() > threshold
                    || (r0c1 - r1c1).abs() > threshold
                    || (r0c1 - r0c2).abs() > threshold
                    || (r0c2 - r0c1).abs() > threshold
                    || (r0c2 - r1c2).abs() > threshold
                    || (r1c0 - r1c1).abs() > threshold
                    || (r1c2 - r1c1).abs() > threshold
                    || (r2c0 - r2c1).abs() > threshold
                    || (r2c0 - r1c0).abs() > threshold
                    || (r2c1 - r2c0).abs() > threshold
                    || (r2c1 - r1c1).abs() > threshold
                    || (r2c1 - r2c2).abs() > threshold
                    || (r2c2 - r2c1).abs() > threshold
                    || (r2c2 - r1c2).abs() > threshold
                {
                    region_possible = false;
                }

                if region_possible {
                    region_sum /= 9;
                    for k in -1..=1i32 {
                        for l in -1..=1i32 {
                            intensity[(i as i32 + k) as usize][(j as i32 + l) as usize] += region_sum;
                            count[(i as i32 + k) as usize][(j as i32 + l) as usize] += 1;
                        }
                    }
                }
            }
        }

        for i in 0..n {
            for j in 0..m {
                if count[i][j] == 0 {
                    intensity[i][j] = image[i][j];
                } else {
                    intensity[i][j] = intensity[i][j] / count[i][j];
                }
            }
        }

        intensity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void resultGrid()
    //   assertThat(
    //   new Solution()
    //   .resultGrid(
    //   CommonUtils
    //   .convertLeetCodeIrregularLengths2DArrayInputIntoJavaArray(
    //   "[5,6,7,10],[8,9,10,10],[11,12,13,10]"),
    //   3),
    //   equalTo(
    //   CommonUtils.convertLeetCodeIrregularLengths2DArrayInputIntoJavaArray(
    //   "[9,9,9,9],[9,9,9,9],[9,9,9,9]")));
    #[test]
    fn test_result_grid() {
        let image = vec![vec![5, 6, 7, 10], vec![8, 9, 10, 10], vec![11, 12, 13, 10]];
        let expected = vec![vec![9, 9, 9, 9], vec![9, 9, 9, 9], vec![9, 9, 9, 9]];
        assert_eq!(Solution::result_grid(image, 3), expected);
    }

    // Java: void resultGrid2()
    //   assertThat(
    //   new Solution()
    //   .resultGrid(
    //   CommonUtils
    //   .convertLeetCodeIrregularLengths2DArrayInputIntoJavaArray(
    //   "[10,20,30],[15,25,35],[20,30,40],[25,35,45]"),
    //   12),
    //   equalTo(
    //   CommonUtils.convertLeetCodeIrregularLengths2DArrayInputIntoJavaArray(
    //   "[25,25,25],[27,27,27],[27,27,27],[30,30,30]")));
    #[test]
    fn test_result_grid2() {
        let image = vec![
            vec![10, 20, 30],
            vec![15, 25, 35],
            vec![20, 30, 40],
            vec![25, 35, 45],
        ];
        let expected = vec![
            vec![25, 25, 25],
            vec![27, 27, 27],
            vec![27, 27, 27],
            vec![30, 30, 30],
        ];
        assert_eq!(Solution::result_grid(image, 12), expected);
    }

    // Java: void resultGrid3()
    //   assertThat(
    //   new Solution()
    //   .resultGrid(
    //   CommonUtils
    //   .convertLeetCodeIrregularLengths2DArrayInputIntoJavaArray(
    //   "[5,6,7],[8,9,10],[11,12,13]"),
    //   1),
    //   equalTo(
    //   CommonUtils.convertLeetCodeIrregularLengths2DArrayInputIntoJavaArray(
    //   "[5,6,7],[8,9,10],[11,12,13]")));
    #[test]
    fn test_result_grid3() {
        let image = vec![vec![5, 6, 7], vec![8, 9, 10], vec![11, 12, 13]];
        let expected = vec![vec![5, 6, 7], vec![8, 9, 10], vec![11, 12, 13]];
        assert_eq!(Solution::result_grid(image, 1), expected);
    }
}
