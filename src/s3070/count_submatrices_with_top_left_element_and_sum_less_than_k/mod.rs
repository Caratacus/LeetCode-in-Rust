// Problem 3070: count submatrices with top left element and sum less than k
// #Medium #Array #Matrix #Prefix_Sum #2024_04_15_Time_2_ms_(100.00%)_Space_117.3_MB_(94.08%)

pub struct Solution;

impl Solution {
    pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = grid[0].len();
        let mut sums = vec![0i64; n];
        let mut ans = 0;
        for row in &grid {
            let mut sum: i64 = 0;
            for col in 0..n {
                sum += row[col] as i64;
                sums[col] += sum;
                if sums[col] <= k as i64 {
                    ans += 1;
                } else {
                    break;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void countSubmatrices()
    //   assertThat(
    //   new Solution()
    //   .countSubmatrices(
    //   CommonUtils
    //   .convertLeetCodeIrregularLengths2DArrayInputIntoJavaArray(
    //   "[7,6,3],[6,6,1]"),
    //   18),
    //   equalTo(4));
    #[test]
    fn test_count_submatrices() {
        assert_eq!(
            Solution::count_submatrices(vec![vec![7, 6, 3], vec![6, 6, 1]], 18),
            4
        );
    }

    // Java: void countSubmatrices2()
    //   assertThat(
    //   new Solution()
    //   .countSubmatrices(
    //   CommonUtils
    //   .convertLeetCodeIrregularLengths2DArrayInputIntoJavaArray(
    //   "[7,2,9],[1,5,0],[2,6,6]"),
    //   20),
    //   equalTo(6));
    #[test]
    fn test_count_submatrices2() {
        assert_eq!(
            Solution::count_submatrices(vec![vec![7, 2, 9], vec![1, 5, 0], vec![2, 6, 6]], 20),
            6
        );
    }
}
