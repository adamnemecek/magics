use crate::{GbpSchedule, GbpScheduleAtIteration, GbpScheduleIterator, GbpScheduleParams};

pub struct Centered;

mod private {
    pub struct CenteredIter {
        n: u8,
        max: u8,
        index: u8,
    }

    impl CenteredIter {
        pub fn new(n: u8, max: u8) -> Self {
            assert!(n <= max, "n must be less than or equal to max");
            Self { n, max, index: 0 }
        }
    }

    impl Iterator for CenteredIter {
        type Item = bool;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index >= self.max {
                return None;
            }
            if self.n == 0 && self.max == 1 {
                self.index += 1;
                return Some(false);
            }

            let mid_point = self.max / 2;
            let half_n = self.n / 2;

            let start = mid_point.saturating_sub(half_n);
            let end = if start + self.n <= self.max {
                start + self.n - 1
            } else {
                self.max - 1
            };

            let result = self.index >= start && self.index <= end;
            self.index += 1;
            Some(result)
        }
    }

    impl ExactSizeIterator for CenteredIter {
        fn len(&self) -> usize {
            self.max as usize
        }
    }
}

pub struct CenteredIter {
    iter: std::iter::Zip<private::CenteredIter, private::CenteredIter>,
}

impl CenteredIter {
    pub fn new(config: GbpScheduleParams) -> Self {
        let max = config.max();
        let internal = private::CenteredIter::new(config.internal, max);
        let external = private::CenteredIter::new(config.external, max);
        let iter = internal.zip(external);
        Self { iter }
    }
}

impl Iterator for CenteredIter {
    type Item = GbpScheduleAtIteration;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(internal, external)| GbpScheduleAtIteration { internal, external })
    }
}

impl ExactSizeIterator for CenteredIter {}

impl GbpScheduleIterator for CenteredIter {}

impl GbpSchedule for Centered {
    fn schedule(config: GbpScheduleParams) -> impl GbpScheduleIterator {
        CenteredIter::new(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const fn ts(internal: bool, external: bool) -> GbpScheduleAtIteration {
        GbpScheduleAtIteration { internal, external }
    }

    #[test]
    fn internal_greater_than_external() {
        let config = GbpScheduleParams {
            internal: 10,
            external: 5,
        };
        let mut schedule = Centered::schedule(config);
        assert_eq!(schedule.next(), Some(ts(true, false)));
        assert_eq!(schedule.next(), Some(ts(true, false)));
        assert_eq!(schedule.next(), Some(ts(true, false)));
        assert_eq!(schedule.next(), Some(ts(true, true)));
        assert_eq!(schedule.next(), Some(ts(true, true)));
        assert_eq!(schedule.next(), Some(ts(true, true)));
        assert_eq!(schedule.next(), Some(ts(true, true)));
        assert_eq!(schedule.next(), Some(ts(true, true)));
        assert_eq!(schedule.next(), Some(ts(true, false)));
        assert_eq!(schedule.next(), Some(ts(true, false)));

        assert_eq!(schedule.next(), None);
    }

    #[test]
    fn internal_less_than_external() {
        let config = GbpScheduleParams {
            internal: 4,
            external: 6,
        };
        let mut schedule = Centered::schedule(config);
        assert_eq!(schedule.next(), Some(ts(false, true)));
        assert_eq!(schedule.next(), Some(ts(true, true)));
        assert_eq!(schedule.next(), Some(ts(true, true)));

        assert_eq!(schedule.next(), Some(ts(true, true)));
        assert_eq!(schedule.next(), Some(ts(true, true)));
        assert_eq!(schedule.next(), Some(ts(false, true)));

        assert_eq!(schedule.next(), None);
    }

    #[test]
    fn internal_external_even() {
        let config = GbpScheduleParams {
            internal: 3,
            external: 3,
        };
        let mut schedule = Centered::schedule(config);
        assert_eq!(schedule.next(), Some(ts(true, true)));
        assert_eq!(schedule.next(), Some(ts(true, true)));
        assert_eq!(schedule.next(), Some(ts(true, true)));
        assert_eq!(schedule.next(), None);
    }

    #[test]
    fn both_zero() {
        let config = GbpScheduleParams {
            internal: 0,
            external: 0,
        };
        let mut schedule = Centered::schedule(config);
        assert_eq!(schedule.next(), None);
    }

    #[test]
    fn internal_zero_external_not() {
        let config = GbpScheduleParams {
            internal: 0,
            external: 1,
        };
        let mut schedule = Centered::schedule(config);
        assert_eq!(schedule.next(), Some(ts(false, true)));
        assert_eq!(schedule.next(), None);

        let config = GbpScheduleParams {
            internal: 0,
            external: 2,
        };
        let mut schedule = Centered::schedule(config);
        assert_eq!(schedule.next(), Some(ts(false, true)));
        assert_eq!(schedule.next(), Some(ts(false, true)));
        assert_eq!(schedule.next(), None);

        let config = GbpScheduleParams {
            internal: 0,
            external: 3,
        };
        let mut schedule = Centered::schedule(config);
        assert_eq!(schedule.next(), Some(ts(false, true)));
        assert_eq!(schedule.next(), Some(ts(false, true)));
        assert_eq!(schedule.next(), Some(ts(false, true)));
        assert_eq!(schedule.next(), None);
    }

    #[test]
    fn external_zero_internal_not() {
        let config = GbpScheduleParams {
            internal: 1,
            external: 0,
        };
        let mut schedule = Centered::schedule(config);
        assert_eq!(schedule.next(), Some(ts(true, false)));
        assert_eq!(schedule.next(), None);

        let config = GbpScheduleParams {
            internal: 2,
            external: 0,
        };
        let mut schedule = Centered::schedule(config);
        assert_eq!(schedule.next(), Some(ts(true, false)));
        assert_eq!(schedule.next(), Some(ts(true, false)));
        assert_eq!(schedule.next(), None);

        let config = GbpScheduleParams {
            internal: 3,
            external: 0,
        };
        let mut schedule = Centered::schedule(config);
        assert_eq!(schedule.next(), Some(ts(true, false)));
        assert_eq!(schedule.next(), Some(ts(true, false)));
        assert_eq!(schedule.next(), Some(ts(true, false)));
        assert_eq!(schedule.next(), None);
    }

    #[test]
    fn both_one() {
        let config = GbpScheduleParams {
            internal: 1,
            external: 1,
        };
        let mut schedule = Centered::schedule(config);
        assert_eq!(schedule.next(), Some(ts(true, true)));
        assert_eq!(schedule.next(), None);
    }

    #[test]
    fn edge_cases() {
        let config = GbpScheduleParams {
            internal: 1,
            external: 2,
        };
        let mut schedule = Centered::schedule(config);
        assert_eq!(schedule.next(), Some(ts(false, true)));
        assert_eq!(schedule.next(), Some(ts(true, true)));
        assert_eq!(schedule.next(), None);

        let config = GbpScheduleParams {
            internal: 3,
            external: 2,
        };
        let mut schedule = Centered::schedule(config);
        assert_eq!(schedule.next(), Some(ts(true, true)));
        assert_eq!(schedule.next(), Some(ts(true, true)));
        assert_eq!(schedule.next(), Some(ts(true, false)));
        assert_eq!(schedule.next(), None);

        let config = GbpScheduleParams {
            internal: 2,
            external: 5,
        };
        let mut schedule = Centered::schedule(config);
        assert_eq!(schedule.next(), Some(ts(false, true)));
        assert_eq!(schedule.next(), Some(ts(true, true)));
        assert_eq!(schedule.next(), Some(ts(true, true)));
        assert_eq!(schedule.next(), Some(ts(false, true)));
        assert_eq!(schedule.next(), Some(ts(false, true)));
        assert_eq!(schedule.next(), None);
    }
}
