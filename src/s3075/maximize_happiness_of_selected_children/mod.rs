// Problem 3075: maximize happiness of selected children
// #Medium #Array #Sorting #Greedy #2024_04_15_Time_34_ms_(97.43%)_Space_61.4_MB_(77.84%)

pub struct Solution;

impl Solution {
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
        happiness.sort();
        let n = happiness.len();
        let mut sum: i64 = 0;
        for i in (n - k as usize)..n {
            let decrease = (n - 1 - i) as i32;
            happiness[i] = std::cmp::max(0, happiness[i] - decrease);
            sum += happiness[i] as i64;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maximumHappinessSum()
    //   assertThat(new Solution().maximumHappinessSum(new int[] {1, 2, 3}, 2), equalTo(4L));
    #[test]
    fn test_maximum_happiness_sum() {
        assert_eq!(Solution::maximum_happiness_sum(vec![1, 2, 3], 2), 4);
    }

    // Java: void maximumHappinessSum2()
    //   assertThat(new Solution().maximumHappinessSum(new int[] {1, 1, 1, 1}, 2), equalTo(1L));
    #[test]
    fn test_maximum_happiness_sum2() {
        assert_eq!(Solution::maximum_happiness_sum(vec![1, 1, 1, 1], 2), 1);
    }

    // Java: void maximumHappinessSum3()
    //   assertThat(new Solution().maximumHappinessSum(new int[] {2, 3, 4, 5}, 1), equalTo(5L));
    #[test]
    fn test_maximum_happiness_sum3() {
        assert_eq!(Solution::maximum_happiness_sum(vec![2, 3, 4, 5], 1), 5);
    }
}
