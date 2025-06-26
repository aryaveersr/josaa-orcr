use std::ops::RangeInclusive;

/// An inclusive range with `u32` bounds.
///
/// Unlike [`RangeInclusive`], it exposes its `start` and `end` bounds.
#[derive(Clone, Copy)]
pub struct RankRange {
    pub start: u32,
    pub end: u32,
}

impl RankRange {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn is_empty(&self) -> bool {
        self.start > self.end
    }

    pub fn contains(&self, value: u32) -> bool {
        self.start <= value && value <= self.end
    }
}

impl Default for RankRange {
    fn default() -> Self {
        Self { start: 4, end: 3 }
    }
}

impl<T> From<RangeInclusive<T>> for RankRange
where
    T: Into<u32> + Clone + Copy,
{
    fn from(value: RangeInclusive<T>) -> Self {
        Self {
            start: (*value.start()).into(),
            end: (*value.end()).into(),
        }
    }
}

impl Iterator for RankRange {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            None
        } else {
            self.start += 1;
            Some(self.start - 1)
        }
    }
}
