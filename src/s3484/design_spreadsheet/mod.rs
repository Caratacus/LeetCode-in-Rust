// Problem 3484: design spreadsheet

pub struct Spreadsheet {
    rows: i32,
}

impl Spreadsheet {
    pub fn new(rows: i32) -> Self {
        todo!()
    }

    pub fn set_cell(&mut self, cell: String, value: i32) -> () {
        todo!()
    }

    pub fn reset_cell(&mut self, cell: String) -> () {
        todo!()
    }

    pub fn get_value(&self, formula: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void spreadsheet()
    //   // Initializes a spreadsheet with 3 rows and 26 columns
    //   Spreadsheet spreadsheet = new Spreadsheet(3);
    //   // returns 12 (5+7)
    //   assertThat(spreadsheet.getValue("=5+7"), equalTo(12));
    //   // sets A1 to 10
    //   ... (11 more lines)
    #[test]
    fn test_spreadsheet() {
        // TODO: 翻译 Java 测试
    }
}
