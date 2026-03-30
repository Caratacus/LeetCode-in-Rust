// Problem 1195: fizz buzz multithreaded

pub struct FizzBuzz {
    n: i32,
}

impl FizzBuzz {
    pub fn new(n: i32) -> Self {
        todo!()
    }

    pub fn fizz(&mut self, print_fizz: Box<dyn FnOnce()>) -> () {
        todo!()
    }

    pub fn buzz(&mut self, print_buzz: Box<dyn FnOnce()>) -> () {
        todo!()
    }

    pub fn fizzbuzz(&mut self, print_fizz_buzz: Box<dyn FnOnce()>) -> () {
        todo!()
    }

    pub fn number(&mut self, print_number: Box<dyn FnMut(i32)>) -> () {
        todo!()
    }
}
