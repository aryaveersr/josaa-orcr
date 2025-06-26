use std::ops::RangeInclusive;

/// An inclusive range with `u32` bounds.
///
/// Unlike [`RangeInclusive`], it exposes its `start` and `end` bounds.
#[derive(Clone, Copy, Debug)]
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

impl PartialEq for RankRange {
    fn eq(&self, other: &Self) -> bool {
        // Either they have the same bounds
        (self.start == other.start && self.end == other.end)
        // or they are both empty
            || (self.start > self.end && other.start > other.end)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank_range_is_empty() {
        // Empty
        assert!(RankRange::default().is_empty());
        assert!(RankRange::new(12, 4).is_empty());
        assert!(RankRange::new(112, 48).is_empty());

        // Not empty
        assert!(!RankRange::new(4, 40).is_empty());
        assert!(!RankRange::new(9, 12).is_empty());
    }

    #[test]
    fn rank_range_contains() {
        let range = RankRange::new(4, 12);

        // In range
        assert!(range.contains(4));
        assert!(range.contains(12));
        assert!(range.contains(6));
        assert!(range.contains(7));

        // Out of range
        assert!(!range.contains(3));
        assert!(!range.contains(13));
        assert!(!range.contains(20));
        assert!(!range.contains(1));
    }

    #[test]
    fn rank_range_equality() {
        // Equal bounds
        assert_eq!(RankRange::new(4, 12), RankRange::new(4, 12));
        assert_eq!(RankRange::new(44, 56), RankRange::new(44, 56));

        // Unequal bounds
        assert_ne!(RankRange::new(12, 24), RankRange::new(12, 23));
        assert_ne!(RankRange::new(12, 24), RankRange::new(12, 27));

        // Both empty
        assert_eq!(RankRange::default(), RankRange::new(6, 4));
        assert_eq!(RankRange::new(12, 8), RankRange::new(83, 40));

        // Only one empty
        assert_ne!(RankRange::new(12, 24), RankRange::default());
        assert_ne!(RankRange::new(24, 12), RankRange::new(83, 123));
    }

    #[test]
    fn rank_range_iter() {
        // Empty
        let mut iter = RankRange::default();

        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let mut iter = RankRange::new(24, 12);

        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        // Non-empty
        let mut iter = RankRange::new(12, 15);

        assert_eq!(iter.next(), Some(12));
        assert_eq!(iter.next(), Some(13));
        assert_eq!(iter.next(), Some(14));
        assert_eq!(iter.next(), Some(15));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
