// Problem 3197: find the minimum area to cover all ones ii
// #Hard #Array #Matrix #Enumeration #2024_06_26_Time_10_ms_(99.66%)_Space_44.1_MB_(85.42%)

pub struct Solution;

impl Solution {
    // rectangle unit count
    fn units_in_rectangle(ruc: &[Vec<i32>], r0: usize, c0: usize, r1: usize, c1: usize) -> i32 {
        ruc[r1][c1] - ruc[r0][c1] - ruc[r1][c0] + ruc[r0][c0]
    }

    fn min_area(ruc: &[Vec<i32>], r0: usize, c0: usize, r1: usize, c1: usize) -> i32 {
        if Self::units_in_rectangle(ruc, r0, c0, r1, c1) == 0 {
            return 0;
        }
        let mut min_row = r0;
        while Self::units_in_rectangle(ruc, r0, c0, min_row + 1, c1) == 0 {
            min_row += 1;
        }
        let mut max_row = r1 - 1;
        while Self::units_in_rectangle(ruc, max_row, c0, r1, c1) == 0 {
            max_row -= 1;
        }
        let mut min_col = c0;
        while Self::units_in_rectangle(ruc, r0, c0, r1, min_col + 1) == 0 {
            min_col += 1;
        }
        let mut max_col = c1 - 1;
        while Self::units_in_rectangle(ruc, r0, max_col, r1, c1) == 0 {
            max_col -= 1;
        }
        ((max_row - min_row + 1) * (max_col - min_col + 1)) as i32
    }

    fn min_sum2(
        ruc: &[Vec<i32>],
        r0: usize,
        c0: usize,
        r1: usize,
        c1: usize,
        split_vertical: bool,
    ) -> i32 {
        let mut min = i32::MAX;
        if split_vertical {
            for c in c0 + 1..c1 {
                let a1 = Self::min_area(ruc, r0, c0, r1, c);
                if a1 == 0 {
                    continue;
                }
                let a2 = Self::min_area(ruc, r0, c, r1, c1);
                if a2 != 0 {
                    min = min.min(a1 + a2);
                }
            }
        } else {
            for r in r0 + 1..r1 {
                let a1 = Self::min_area(ruc, r0, c0, r, c1);
                if a1 == 0 {
                    continue;
                }
                let a2 = Self::min_area(ruc, r, c0, r1, c1);
                if a2 != 0 {
                    min = min.min(a1 + a2);
                }
            }
        }
        min
    }

    fn min_sum3(
        ruc: &[Vec<i32>],
        height: usize,
        width: usize,
        first_split_vertical: bool,
        take_lower: bool,
        second_split_vertical: bool,
    ) -> i32 {
        let mut min = i32::MAX;
        if first_split_vertical {
            for c in 1..width {
                let a1;
                let a2;
                if take_lower {
                    a1 = Self::min_area(ruc, 0, 0, height, c);
                    if a1 == 0 {
                        continue;
                    }
                    a2 = Self::min_sum2(ruc, 0, c, height, width, second_split_vertical);
                } else {
                    a1 = Self::min_area(ruc, 0, c, height, width);
                    if a1 == 0 {
                        continue;
                    }
                    a2 = Self::min_sum2(ruc, 0, 0, height, c, second_split_vertical);
                }
                if a2 != i32::MAX {
                    min = min.min(a1 + a2);
                }
            }
        } else {
            for r in 1..height {
                let a1;
                let a2;
                if take_lower {
                    a1 = Self::min_area(ruc, 0, 0, r, width);
                    if a1 == 0 {
                        continue;
                    }
                    a2 = Self::min_sum2(ruc, r, 0, height, width, second_split_vertical);
                } else {
                    a1 = Self::min_area(ruc, r, 0, height, width);
                    if a1 == 0 {
                        continue;
                    }
                    a2 = Self::min_sum2(ruc, 0, 0, r, width, second_split_vertical);
                }
                if a2 != i32::MAX {
                    min = min.min(a1 + a2);
                }
            }
        }
        min
    }

    pub fn minimum_sum(grid: Vec<Vec<i32>>) -> i32 {
        let height = grid.len();
        let width = grid[0].len();
        let mut ruc = vec![vec![0; width + 1]; height + 1];
        for i in 0..height {
            let mut c = 0;
            for j in 0..width {
                c += grid[i][j];
                ruc[i + 1][j + 1] = ruc[i][j + 1] + c;
            }
        }
        let mut min = i32::MAX;
        min = min.min(Self::min_sum3(&ruc, height, width, true, true, true));
        min = min.min(Self::min_sum3(&ruc, height, width, true, true, false));
        min = min.min(Self::min_sum3(&ruc, height, width, true, false, false));
        min = min.min(Self::min_sum3(&ruc, height, width, false, true, true));
        min = min.min(Self::min_sum3(&ruc, height, width, false, true, false));
        min = min.min(Self::min_sum3(&ruc, height, width, false, false, true));
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minimumSum()
    //   assertThat(new Solution().minimumSum(new int[][] {{1, 0, 1}, {1, 1, 1}}), equalTo(5));
    #[test]
    fn test_minimum_sum() {
        assert_eq!(Solution::minimum_sum(vec![vec![1, 0, 1], vec![1, 1, 1]]), 5);
    }

    // Java: void minimumSum2()
    //   assertThat(new Solution().minimumSum(new int[][] {{1, 0, 1, 0}, {0, 1, 0, 1}}), equalTo(5));
    #[test]
    fn test_minimum_sum2() {
        assert_eq!(
            Solution::minimum_sum(vec![vec![1, 0, 1, 0], vec![0, 1, 0, 1]]),
            5
        );
    }
}
