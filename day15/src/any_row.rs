use crate::range::Range;

pub struct AnyRow {
    pub ranges: Vec<Range>,
    pub min: usize,
    pub max: usize,
}

impl AnyRow {
    pub fn new(min: usize, max: usize) -> Self {
        Self {
            ranges: Vec::new(),
            min,
            max,
        }
    }

    pub fn add_range(&mut self, position: i64, traversal: i64) {
        let range: Range = Range {
            from: (position - traversal).clamp(self.min as i64, self.max as i64) as usize,
            to: (position + traversal).clamp(self.min as i64, self.max as i64) as usize,
        };
        self.internal_add_range(range);
    }

    fn internal_add_range(&mut self, range: Range) {
        self.ranges.push(range);

        if self.ranges.len() > 1 {
            self.sort_and_merge();
        }
    }

    fn sort_and_merge(&mut self) {
        // need to compare every range with the other as we add in no order
        self.ranges.sort_by(|a, b| a.from.cmp(&b.from));

        let mut index = 0;
        while index < self.ranges.len() - 1 {
            let b = self.ranges.remove(index + 1);
            let a = self.ranges.remove(index);
            match Range::merge_ranges(a, b) {
                (None, None) => panic!("Come look at this, you messed up good on the merging."),
                (None, Some(_merged)) => (), // nevewr going to happen...
                (Some(merged), None) => {
                    self.ranges.insert(index, merged);
                    if self.ranges.len() > index + 1 {
                        continue; // i.e. skip increment of index
                    }
                }
                (Some(orig_a), Some(orig_b)) => {
                    self.ranges.insert(index, orig_a);
                    self.ranges.insert(index + 1, orig_b);
                }
            }
            index += 1;
        }
    }

    pub fn is_there_a_gap(&mut self) -> bool {
        self.ranges.len() > 1
    }

    pub fn what_is_that_gap(&mut self) -> usize {
        self.ranges.pop().unwrap().from - 1
    }
}
