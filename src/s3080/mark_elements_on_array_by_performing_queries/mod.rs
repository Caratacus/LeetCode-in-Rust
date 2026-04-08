// Problem 3080: mark elements on array by performing queries
// #Medium #Array #Hash_Table #Sorting #Heap_Priority_Queue #Simulation

pub struct Solution;

impl Solution {
    pub fn unmarked_sum_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let l = nums.len();
        let mut nums = nums;
        let mut orig: Vec<usize> = (0..l).collect();

        // Merge sort to sort nums while tracking original indices
        let mut x = 1;
        while x < l {
            let mut temp = vec![0i32; l];
            let mut teor = vec![0usize; l];
            let mut y = 0;
            while y < l {
                let mut s1 = 0;
                let mut s2 = 0;
                while s1 + s2 < 2 * x && y + s1 + s2 < l {
                    if s2 >= x || y + x + s2 >= l {
                        temp[y + s1 + s2] = nums[y + s1];
                        teor[y + s1 + s2] = orig[y + s1];
                        s1 += 1;
                    } else if s1 >= x {
                        temp[y + s1 + s2] = nums[y + x + s2];
                        teor[y + s1 + s2] = orig[y + x + s2];
                        s2 += 1;
                    } else if nums[y + s1] <= nums[y + x + s2] {
                        temp[y + s1 + s2] = nums[y + s1];
                        teor[y + s1 + s2] = orig[y + s1];
                        s1 += 1;
                    } else {
                        temp[y + s1 + s2] = nums[y + x + s2];
                        teor[y + s1 + s2] = orig[y + x + s2];
                        s2 += 1;
                    }
                }
                y += 2 * x;
            }
            for i in 0..l {
                nums[i] = temp[i];
                orig[i] = teor[i];
            }
            x *= 2;
        }

        let mut change = vec![0usize; l];
        for i in 0..l {
            change[orig[i]] = i;
        }

        let mut mark = vec![false; l];
        let m = queries.len();
        let mut st = 0;
        let mut sum: i64 = nums.iter().map(|&x| x as i64).sum();
        let mut out = vec![0i64; m];

        for i in 0..m {
            let a = queries[i][0] as usize;
            if !mark[change[a]] {
                mark[change[a]] = true;
                sum -= nums[change[a]] as i64;
            }
            let b = queries[i][1] as usize;
            let mut many = 0;
            while many < b {
                if st == l {
                    out[i] = sum;
                    break;
                }
                if !mark[st] {
                    mark[st] = true;
                    sum -= nums[st] as i64;
                    many += 1;
                }
                st += 1;
            }
            out[i] = sum;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void unmarkedSumArray()
    //   assertThat(
    //   new Solution()
    //   .unmarkedSumArray(
    //   new int[] {1, 2, 2, 1, 2, 3, 1},
    //   CommonUtils.convertLeetCodeIrregularLengths2DArrayInputIntoJavaArray("[1,2],[3,3],[4,2]")),
    //   equalTo(new long[] {8, 3, 0}));
    #[test]
    fn test_unmarked_sum_array() {
        assert_eq!(
            Solution::unmarked_sum_array(vec![1, 2, 2, 1, 2, 3, 1], vec![vec![1, 2], vec![3, 3], vec![4, 2]]),
            vec![8, 3, 0]
        );
    }

    // Java: void unmarkedSumArray2()
    //   assertThat(
    //   new Solution().unmarkedSumArray(new int[] {1, 4, 2, 3}, new int[][] {{0, 1}}),
    //   equalTo(new long[] {7}));
    #[test]
    fn test_unmarked_sum_array2() {
        assert_eq!(
            Solution::unmarked_sum_array(vec![1, 4, 2, 3], vec![vec![0, 1]]),
            vec![7]
        );
    }
}
