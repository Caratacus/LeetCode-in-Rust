// Problem 3186: maximum total damage with spell casting
// #Medium #Array #Hash_Table #Dynamic_Programming #Sorting #Binary_Search #Two_Pointers #Counting
// #2024_06_21_Time_51_ms_(99.29%)_Space_60.8_MB_(78.34%)

pub struct Solution;

impl Solution {
    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        let max_power = *power.iter().max().unwrap_or(&0);
        if max_power <= 1_000_000 {
            Self::small_power(&power, max_power)
        } else {
            Self::big_power(&power)
        }
    }

    fn small_power(power: &[i32], max_power: i32) -> i64 {
        let mut counts = vec![0i64; (max_power + 6) as usize];
        for &p in power {
            counts[p as usize] += 1;
        }
        let mut dp = vec![0i64; (max_power + 6) as usize];
        dp[1] = counts[1];
        dp[2] = (counts[2] * 2).max(dp[1]);
        for i in 3..=max_power as usize {
            dp[i] = (counts[i] * i as i64 + dp[i - 3]).max(dp[i - 1].max(dp[i - 2]));
        }
        dp[max_power as usize]
    }

    fn big_power(power: &[i32]) -> i64 {
        let mut power = power.to_vec();
        power.sort();
        let n = power.len();
        let mut prevs = vec![0i64; 4];
        let mut cur_power = power[0];
        let mut count: i64 = 1;
        let mut result: i64 = 0;
        for i in 1..=n {
            let p = if i == n { 1_000_000_009 } else { power[i] };
            if p == cur_power {
                count += 1;
            } else {
                let cur_val = ((cur_power as i64) * count + prevs[3])
                    .max(prevs[1].max(prevs[2]));
                let diff = std::cmp::min(p - cur_power, prevs.len() as i32 - 1);
                let next_cur_val = if diff == 1 {
                    0
                } else {
                    prevs[3].max(cur_val.max(prevs[2]))
                };
                // Shift the values in prevs[].
                let mut k = prevs.len() - 1;
                if diff < prevs.len() as i32 - 1 {
                    while k > diff as usize {
                        prevs[k] = prevs[k - diff as usize];
                        k -= 1;
                    }
                    prevs[k] = cur_val;
                    k -= 1;
                }
                while k > 0 {
                    prevs[k] = next_cur_val;
                    k -= 1;
                }
                cur_power = p;
                count = 1;
            }
        }
        for v in prevs {
            if v > result {
                result = v;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maximumTotalDamage()
    //   assertThat(new Solution().maximumTotalDamage(new int[] {1, 1, 3, 4}), equalTo(6L));
    #[test]
    fn test_maximum_total_damage() {
        assert_eq!(Solution::maximum_total_damage(vec![1, 1, 3, 4]), 6);
    }

    // Java: void maximumTotalDamage2()
    //   assertThat(new Solution().maximumTotalDamage(new int[] {7, 1, 6, 6}), equalTo(13L));
    #[test]
    fn test_maximum_total_damage2() {
        assert_eq!(Solution::maximum_total_damage(vec![7, 1, 6, 6]), 13);
    }

    // Java: void maximumTotalDamage3()
    //   assertThat(
    //   new Solution().maximumTotalDamage(new int[] {1_000_001, 1, 6, 6}),
    //   equalTo(1000014L));
    #[test]
    fn test_maximum_total_damage3() {
        assert_eq!(
            Solution::maximum_total_damage(vec![1_000_001, 1, 6, 6]),
            1000014
        );
    }
}
