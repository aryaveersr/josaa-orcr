use crate::Filters;

pub struct Entry {
    pub institute: String,
    pub branch: String,
    pub quota: String,
    pub seat_type: String,
    pub gender: String,
    pub or: u32,
    pub cr: u32,
}

pub struct EntryIterator<'a> {
    filters: &'a Filters,
    entries: &'a [Entry],
}

impl<'a> EntryIterator<'a> {
    pub fn new(filters: &'a Filters, entries: &'a [Entry]) -> Self {
        Self { filters, entries }
    }
}

impl<'a> Iterator for EntryIterator<'a> {
    type Item = &'a Entry;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entry = self.entries.get(0)?;
            self.entries = &self.entries[1..];

            if self.filters.matches(entry) {
                return Some(entry);
            }
        }
    }
}
