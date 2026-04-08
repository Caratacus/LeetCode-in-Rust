// Problem 3147: taking maximum energy from the mystic dungeon
// #Medium #Array #Prefix_Sum #2024_05_15_Time_2_ms_(97.58%)_Space_59.8_MB_(75.38%)

pub struct Solution;

impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let mut max = i32::MIN;
        let n = energy.len();
        let k = k as usize;
        for i in (n - k..n).rev() {
            let mut en = 0;
            let mut j = i as i64;
            while j >= 0 {
                en += energy[j as usize];
                max = max.max(en);
                j -= k as i64;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_energy() {
        assert_eq!(
            Solution::maximum_energy(vec![5, 2, -10, -5, 1], 3),
            3
        );
    }

    #[test]
    fn test_maximum_energy2() {
        assert_eq!(Solution::maximum_energy(vec![-2, -3, -1], 2), -1);
    }
}
