// Problem 2286: booking concert tickets in groups

pub struct BookMyShow {
    n: usize,
    m: i64,
    seats: Vec<i64>,
}

impl BookMyShow {
    pub fn new(n: i32, m: i32) -> Self {
        let n = n as usize;
        let m = m as i64;
        Self {
            n,
            m,
            seats: vec
![m; n],
        }
    }

    pub fn gather(&mut self, k: i32, max_row: i32) -> Vec<i32> {
        let k = k as i64;
        let max_row = (max_row as usize).min(self.n - 1);
        for row in 0..=max_row {
            if self.seats[row] >= k {
                let seat = self.m - self.seats[row];
                self.seats[row] -= k;
                return vec
![row as i32, seat as i32];
            }
        }
        vec
![]
    }

    pub fn scatter(&mut self, k: i32, max_row: i32) -> bool {
        let k = k as i64;
        let max_row = (max_row as usize).min(self.n - 1);
        let total: i64 = self.seats[0..=max_row].iter().sum();
        if total < k {
            return false;
        }
        let mut remaining = k;
        for row in 0..=max_row {
            if remaining <= 0 {
                break;
            }
            let take = self.seats[row].min(remaining);
            self.seats[row] -= take;
            remaining -= take;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_my_show() {
        let mut bms = BookMyShow::new(2, 5);
        assert_eq!(bms.gather(4, 0), vec
![0, 0]);
        assert_eq!(bms.gather(2, 0), vec
![0, 3]);
        assert_eq!(bms.gather(3, 0), vec
![]);
        assert_eq!(bms.scatter(5, 1), true);
        assert_eq!(bms.scatter(5, 1), false);
        assert_eq!(bms.gather(4, 0), vec
![]);
        assert_eq!(bms.scatter(0, 1), true);
        assert_eq!(bms.gather(1, 0), vec
![]);
        assert_eq!(bms.scatter(2, 1), true);
    }

    #[test]
    fn test_book_my_show2() {
        let mut bms = BookMyShow::new(2, 6);
        assert_eq!(bms.scatter(2, 1), true);
        assert_eq!(bms.scatter(8, 0), false);
    }
}
