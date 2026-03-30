// Problem 1117: building h2o

pub struct H2O {}

impl H2O {
    pub fn new() -> Self {
        todo!()
    }

    pub fn hydrogen(&mut self, release_hydrogen: Box<dyn FnOnce()>) -> () {
        todo!()
    }

    pub fn oxygen(&mut self, release_oxygen: Box<dyn FnOnce()>) -> () {
        todo!()
    }

    pub fn run(&mut self) -> () {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void h20()
    //   H2O h2o = new H2O();
    //   ThreadPoolExecutor executor =
    //   new ThreadPoolExecutor(6, 30, 15, TimeUnit.SECONDS, new ArrayBlockingQueue<>(12));
    //   for (int i = 0; i < 12; i++) {
    //   executor.execute(new H2O.HydrogenRunnable(h2o));
    //   ... (4 more lines)
    #[test]
    fn test_h20() {
        // TODO: 翻译 Java 测试
    }
}
