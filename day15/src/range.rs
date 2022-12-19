#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub from: usize,
    pub to: usize,
}

impl Range {
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }

    pub fn merge_ranges(a: Range, b: Range) -> (Option<Range>, Option<Range>) {
        if a.from <= b.from {
            if a.to < b.from {
                //  check contiguous e.g. 1-10 and 11-20 would be 1 - 20
                if a.to + 1 == b.from {
                    return (Some(Range::new(a.from, b.to)), None);
                }
                // a completely under a e.g. a: 1-10, b: 20-30sss
                return (Some(a), Some(b));
            }
            if a.to <= b.to {
                return (Some(Range::new(a.from, b.to)), None);
            }
            return (Some(a), None);
        }

        if a.to >= b.to {
            if a.from > b.to {
                if b.to + 1 == a.from {
                    return (Some(Range::new(b.from, a.to)), None);
                }
                return (Some(a), Some(b));
            }

            if a.from > b.from {
                return (Some(Range::new(b.from, a.to)), None);
            }
            return (Some(b), None);
        }
        // not sure how you'd get here... Rust needs a value though.
        (None, None)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn range_impl_merge_completely_distinct_under() {
        let base = Range::new(10, 20);
        let under = Range::new(2, 8);

        let (a, b) = Range::merge_ranges(base, under);

        assert_eq!(a, Some(base));
        assert_eq!(b, Some(under));
    }
    #[test]
    fn range_impl_merge_completely_distinct_over() {
        let over = Range::new(10, 20);
        let base = Range::new(2, 8);

        let (a, b) = Range::merge_ranges(base, over);

        assert_eq!(a, Some(base));
        assert_eq!(b, Some(over));
    }

    #[test]
    fn range_impl_merge_overlap_under() {
        let base = Range::new(10, 20);
        let under = Range::new(5, 15);

        let (a, b) = Range::merge_ranges(base, under);

        assert_eq!(a, Some(Range::new(5, 20)));
        assert_eq!(b, None);
    }

    #[test]
    fn range_impl_merge_overlap_over() {
        let base = Range::new(10, 20);
        let over = Range::new(15, 25);
        let (a, b) = Range::merge_ranges(base, over);

        assert_eq!(a, Some(Range::new(10, 25)));
        assert_eq!(b, None);
    }

    #[test]
    fn range_impl_merge_wholly_encompass() {
        let base = Range::new(10, 20);
        let within = Range::new(13, 15);
        let (a, b) = Range::merge_ranges(base, within);

        assert_eq!(a, Some(base));
        assert_eq!(b, None);
    }

    #[test]
    fn range_impl_merge_distinct_but_contiguous() {
        let base = Range::new(10, 20);
        let next = Range::new(21, 25);
        let (a, b) = Range::merge_ranges(base, next);

        assert_eq!(a, Some(Range::new(10, 25)));
        assert_eq!(b, None);
    }
}
