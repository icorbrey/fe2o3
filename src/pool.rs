pub struct ThreadPool;

impl ThreadPool {
    pub fn new(count: usize) -> ThreadPool {
        assert!(0 < count);
        ThreadPool
    }
}

#[cfg(test)]
mod should {
    use super::*;
    #[test]
    fn exist() {
        ThreadPool::new(5);
    }

    #[test]
    #[should_panic]
    fn not_allow_zero_workers() {
        ThreadPool::new(0);
    }
}
