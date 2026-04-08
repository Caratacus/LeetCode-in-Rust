// Problem 3044: Most Frequent Prime
// #Medium #Array #Hash_Table #Math #Matrix #Counting #Enumeration #Number_Theory
// #Big_O_Time_O(m*n*max(m,n))_Space_O(m*n)

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn most_frequent_prime(mat: Vec<Vec<i32>>) -> i32 {
        let directions: [(i32, i32); 8] = [
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
        ];
        let m = mat.len();
        let n = mat[0].len();
        let mut prime_freq: HashMap<i32, i32> = HashMap::new();
        let mut max_count = 0;
        let mut freq_num = -1;

        for i in 0..m {
            for j in 0..n {
                for &(dx, dy) in &directions {
                    Self::get_prime(
                        i as i32,
                        j as i32,
                        &mat,
                        0,
                        dx,
                        dy,
                        &mut prime_freq,
                        &mut max_count,
                        &mut freq_num,
                    );
                }
            }
        }
        freq_num
    }

    fn get_prime(
        i: i32,
        j: i32,
        mat: &Vec<Vec<i32>>,
        num: i32,
        dx: i32,
        dy: i32,
        prime_freq: &mut HashMap<i32, i32>,
        max_count: &mut i32,
        freq_num: &mut i32,
    ) {
        let m = mat.len() as i32;
        let n = mat[0].len() as i32;
        if i < 0 || j < 0 || i >= m || j >= n {
            return;
        }
        let num = num * 10 + mat[i as usize][j as usize];
        if num > 10 && Self::is_prime(num) {
            let count = prime_freq.get(&num).unwrap_or(&0) + 1;
            if (count == *max_count && num > *freq_num) || count > *max_count {
                *freq_num = num;
            }
            *max_count = (*max_count).max(count);
            prime_freq.insert(num, count);
        }
        Self::get_prime(i + dx, j + dy, mat, num, dx, dy, prime_freq, max_count, freq_num);
    }

    fn is_prime(num: i32) -> bool {
        if num == 2 {
            return true;
        }
        if num == 1 || num % 2 == 0 {
            return false;
        }
        let n = (num as f64).sqrt() as i32;
        for i in (3..=n).step_by(2) {
            if num % i == 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_frequent_prime() {
        assert_eq!(
            Solution::most_frequent_prime(vec![vec![1, 1], vec![9, 9], vec![1, 1]]),
            19
        );
    }

    #[test]
    fn test_most_frequent_prime2() {
        assert_eq!(Solution::most_frequent_prime(vec![vec![7]]), -1);
    }

    #[test]
    fn test_most_frequent_prime3() {
        assert_eq!(
            Solution::most_frequent_prime(vec![
                vec![9, 7, 8],
                vec![4, 6, 5],
                vec![2, 8, 6]
            ]),
            97
        );
    }
}
