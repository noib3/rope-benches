use std::ops::Range;

pub const TINY: &str = include_str!("tiny.txt");
pub const SMALL: &str = include_str!("small.txt");
pub const MEDIUM: &str = include_str!("medium.txt");
pub const LARGE: &str = include_str!("large.txt");

#[derive(Debug, Clone)]
pub struct PercentRanges {
    start: usize,
    end: usize,
    half_percent: usize,
}

impl PercentRanges {
    #[allow(dead_code)]
    #[inline]
    pub fn new(max: usize) -> Self {
        Self {
            start: 0,
            end: max,
            half_percent: (max / 200).min(1),
        }
    }
}

impl Iterator for PercentRanges {
    type Item = Range<usize>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            return None;
        }

        let range = self.start..self.end;

        self.end -= self.half_percent;

        self.start = std::cmp::min(self.end, self.start + self.half_percent);

        Some(range)
    }
}
