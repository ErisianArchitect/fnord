use fnord::core::geometry::Cardinal;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Counter(usize);

impl Counter {
    #[must_use]
    #[inline(always)]
    pub const fn new() -> Self {
        Self(0)
    }
    
    #[inline(always)]
    pub const fn increment(&mut self) -> usize {
        let next = self.0;
        self.0 += 1;
        next
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn eq(self, count: usize) -> bool {
        self.0 == count
    }
    
    #[track_caller]
    pub fn assert_eq(self, count: usize) {
        assert_eq!(self.0, count)
    }
    
    #[track_caller]
    pub fn assert_eq_with_msg<M: ::core::fmt::Display>(self, count: usize, msg: M) {
        assert_eq!(self.0, count, "{}", msg)
    }
    
    #[track_caller]
    pub fn count_test<F: FnOnce()>(&mut self, test: F) -> usize {
        let next = self.0;
        test();
        self.0 += 1;
        next
    }
}

#[test]
fn rank_test() {
    let mut proof_of_work = Counter::new();
    macro_rules! primary {
        ($dir:ident) => {
            {
                assert!(Cardinal::$dir.is_primary(), "{} was not primary.", stringify!($dir));
                assert!(!Cardinal::$dir.is_secondary(), "{} was secondary.", stringify!($dir));
                proof_of_work.increment();
            }
        };
        ($first:ident $($dir:ident)+) => {
            {
                primary!($first);
                $(
                    primary!($dir);
                )*
            }
        };
    }
    macro_rules! secondary {
        ($dir:ident) => {
            {
                assert!(Cardinal::$dir.is_secondary(), "{} was not secondary.", stringify!($dir));
                assert!(!Cardinal::$dir.is_primary(), "{} was primary.", stringify!($dir));
                proof_of_work.increment();
            }
        };
        ($first:ident $($dir:ident)+) => {
            {
                secondary!($first);
                $(
                    secondary!($dir);
                )*
            }
        };
    }
    primary!(N W S E);
    secondary!(Ne Nw Se Sw);
    proof_of_work.assert_eq_with_msg(8, "Expected 8 tests.");
}