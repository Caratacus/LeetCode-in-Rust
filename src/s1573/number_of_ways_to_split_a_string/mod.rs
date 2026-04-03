// Problem 1573: Number of Ways to Split a String
// #Medium #String #Math
// #Big_O_Time_O(n)_Space_O(1)

pub struct Solution;

impl Solution {
    pub fn num_ways(s: String) -> i32 {
        let mod_val: i64 = 1_000_000_007;
        let n = s.len() as i64;

        let total_ones_count = s.chars().filter(|&c| c == '1').count() as i64;

        if total_ones_count % 3 != 0 {
            return 0;
        }

        let ones_first_part = total_ones_count / 3;
        let ones_second_part = ones_first_part * 2;

        if total_ones_count == 0 {
            return (((n - 1) * (n - 2) / 2) % mod_val) as i32;
        }

        let mut ways_of_first_string = 0i64;
        let mut ways_of_second_string = 0i64;
        let mut ones_count = 0i64;

        for c in s.chars() {
            if c == '1' {
                ones_count += 1;
            }
            if ones_count == ones_first_part {
                ways_of_first_string += 1;
            } else if ones_count == ones_second_part {
                ways_of_second_string += 1;
            } else if ones_count > ones_second_part {
                break;
            }
        }

        ((ways_of_first_string * ways_of_second_string) % mod_val) as i32
    }
}
