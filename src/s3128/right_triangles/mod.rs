// Problem 3128: right triangles
// #Medium #Array #Hash_Table #Math #Counting #Combinatorics
// #2024_05_02_Time_6 ms(100.00%)  Space 145.9 MB (90.67%)

pub struct Solution;

impl Solution {
    pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid.len();
        let m = grid[0].len();
        let mut columns = vec![0i32; n];
        let mut rows = vec![0i32; m];
        let mut sum: i64 = 0;

        for i in 0..n {
            for j in 0..m {
                columns[i] += grid[i][j];
                rows[j] += grid[i][j];
            }
        }

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    sum += (grid[i][j] as i64) * ((rows[j] - 1) as i64) * ((columns[i] - 1) as i64);
                }
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_right_triangles() {
        assert_eq!(Solution::number_of_right_triangles(vec![vec![0, 1, 0], vec![0, 1, 1], vec![0, 1, 0]]), 2);
    }

    #[test]
    fn test_number_of_right_triangles2() {
        assert_eq!(Solution::number_of_right_triangles(vec![vec![1, 0, 0, 0], vec![0, 1, 0, 1], vec![1, 0, 0, 0]]), 0);
    }

    #[test]
    fn test_number_of_right_triangles3() {
        assert_eq!(Solution::number_of_right_triangles(vec![vec![1, 0, 1], vec![1, 0, 0], vec![1, 0, 0]]), 2);
    }
}
