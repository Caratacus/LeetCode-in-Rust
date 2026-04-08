// Problem 3071: minimum operations to write the letter y on a grid
// #Medium #Array #Hash_Table #Matrix #Counting #2024_04_15_Time_1_ms_(100.00%)_Space_45_MB_(60.73%)

pub struct Solution;

impl Solution {
    pub fn minimum_operations_to_write_y(arr: Vec<Vec<i32>>) -> i32 {
        let n = arr.len();
        let mut cnt1 = [0i32; 3];
        let mut cnt2 = [0i32; 3];
        let mut arr = arr;
        let x = n / 2;
        let y = n / 2;

        // Vertical part of Y (from center to bottom)
        for j in x..n {
            cnt1[arr[j][y] as usize] += 1;
            arr[j][y] = 3;
        }

        // Left diagonal part of Y (from center to top-left)
        for j in (0..=x).rev() {
            if arr[j][j] != 3 {
                cnt1[arr[j][j] as usize] += 1;
            }
            arr[j][j] = 3;
        }

        // Right diagonal part of Y (from center to top-right)
        for j in (0..=x).rev() {
            if arr[j][n - 1 - j] != 3 {
                cnt1[arr[j][n - 1 - j] as usize] += 1;
            }
            arr[j][n - 1 - j] = 3;
        }

        // Count non-Y cells
        for row in &arr {
            for j in 0..n {
                if row[j] != 3 {
                    cnt2[row[j] as usize] += 1;
                }
            }
        }

        let s1 = cnt1[0] + cnt1[1] + cnt1[2];
        let s2 = cnt2[0] + cnt2[1] + cnt2[2];

        let mut min = i32::MAX;
        for i in 0..=2 {
            for j in 0..=2 {
                if i != j {
                    min = std::cmp::min(s1 - cnt1[i] + s2 - cnt2[j], min);
                }
            }
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minimumOperationsToWriteY()
    //   assertThat(
    //   new Solution()
    //   .minimumOperationsToWriteY(
    //   CommonUtils
    //   .convertLeetCodeIrregularLengths2DArrayInputIntoJavaArray(
    //   "[1,2,2],[1,1,0],[0,1,0]")),
    //   equalTo(3));
    #[test]
    fn test_minimum_operations_to_write_y() {
        assert_eq!(
            Solution::minimum_operations_to_write_y(vec![vec![1, 2, 2], vec![1, 1, 0], vec![0, 1, 0]]),
            3
        );
    }

    // Java: void minimumOperationsToWriteY2()
    //   assertThat(
    //   new Solution()
    //   .minimumOperationsToWriteY(
    //   CommonUtils
    //   .convertLeetCodeIrregularLengths2DArrayInputIntoJavaArray(
    //   "[0,1,0,1,0],[2,1,0,1,2],[2,2,2,0,1],[2,2,2,2,2],[2,1,2,2,2]")),
    //   equalTo(12));
    #[test]
    fn test_minimum_operations_to_write_y2() {
        assert_eq!(
            Solution::minimum_operations_to_write_y(vec![
                vec![0, 1, 0, 1, 0],
                vec![2, 1, 0, 1, 2],
                vec![2, 2, 2, 0, 1],
                vec![2, 2, 2, 2, 2],
                vec![2, 1, 2, 2, 2]
            ]),
            12
        );
    }
}
