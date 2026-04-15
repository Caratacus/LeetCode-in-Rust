// Problem 3370: Smallest Number With All Set Bits
// #Easy #Math #Bit_Manipulation

pub struct Solution;

impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let mut res = 1;
        while res < n {
            res = res * 2 + 1;
        }
        res
    }
}
