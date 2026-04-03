// Problem 1582: Special Positions in a Binary Matrix
// #Easy #Array #Matrix
// #Big_O_Time_O(m*n)_Space_O(m+n)

pub struct Solution;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut row_sum = vec![0i32; m];
        let mut col_sum = vec![0i32; n];

        for i in 0..m {
            for j in 0..n {
                row_sum[i] += mat[i][j];
                col_sum[j] += mat[i][j];
            }
        }

        let mut count = 0;
        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 1 && row_sum[i] == 1 && col_sum[j] == 1 {
                    count += 1;
                }
            }
        }
        count
    }
}
