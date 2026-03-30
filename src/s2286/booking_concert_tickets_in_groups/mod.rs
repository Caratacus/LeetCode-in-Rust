// Problem 2286: booking concert tickets in groups

pub struct BookMyShow {
    n: i32,
    m: i32,
}

impl BookMyShow {
    pub fn new(n: i32, m: i32) -> Self {
        todo!()
    }

    pub fn gather(&mut self, k: i32, max_row: i32) -> Vec<i32> {
        todo!()
    }

    pub fn scatter(&mut self, k: i32, max_row: i32) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void bookMyShow()
    //   // There are 2 rows with 5 seats each
    //   BookMyShow bms = new BookMyShow(2, 5);
    //   // return [0, 0]
    //   assertThat(bms.gather(4, 0), equalTo(new int[] {0, 0}));
    //   // The group books seats [0, 3] of row 0.
    //   ... (10 more lines)
    #[test]
    fn test_book_my_show() {
        // TODO: 翻译 Java 测试
    }

    // Java: void bookMyShow2()
    //   // There are 2 rows with 6 seats each
    //   BookMyShow bms = new BookMyShow(2, 6);
    //   assertThat(bms.scatter(2, 1), equalTo(true));
    //   assertThat(bms.scatter(8, 0), equalTo(false));
    #[test]
    fn test_book_my_show2() {
        // TODO: 翻译 Java 测试
    }
}
