// Problem 3363: Find the Maximum Number of Fruits Collected
// #Hard #Array #Dynamic_Programming #Matrix

pub struct Solution;

impl Solution {
    pub fn max_collected_fruits(mut fruits: Vec<Vec<i32>>) -> i32 {
        let n = fruits.len();
        // Set inaccessible cells to 0
        for i in 0..n {
            for j in 0..n {
                if i < j && j < n - 1 - i {
                    fruits[i][j] = 0;
                }
                if j < i && i < n - 1 - j {
                    fruits[i][j] = 0;
                }
            }
        }
        let mut res = 0;
        for i in 0..n {
            res += fruits[i][i];
        }
        // Process upper-right triangle
        for i in 1..n {
            for j in (i + 1)..n {
                let mut max_val = fruits[i - 1][j - 1];
                max_val = std::cmp::max(max_val, fruits[i - 1][j]);
                if j + 1 < n {
                    max_val = std::cmp::max(max_val, fruits[i - 1][j + 1]);
                }
                fruits[i][j] += max_val;
            }
        }
        // Process lower-left triangle
        for j in 1..n {
            for i in (j + 1)..n {
                let mut max_val = fruits[i - 1][j - 1];
                max_val = std::cmp::max(max_val, fruits[i][j - 1]);
                if i + 1 < n {
                    max_val = std::cmp::max(max_val, fruits[i + 1][j - 1]);
                }
                fruits[i][j] += max_val;
            }
        }
        res + fruits[n - 1][n - 2] + fruits[n - 2][n - 1]
    }
}
