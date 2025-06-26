mod entry;
mod filters;
mod options;
mod sort;

pub use entry::*;
pub use filters::*;
pub use options::*;
pub use sort::*;

use rusqlite::{Connection, OpenFlags};

#[derive(Default)]
pub struct Dataset {
    connection: Option<Connection>,
    connection_options: Options,

    entries: Vec<Entry>,
    filters: Filters,
}

impl Dataset {
    pub fn is_loaded(&self) -> bool {
        self.connection.is_some()
    }

    pub fn load(&mut self, options: &Options) -> rusqlite::Result<()> {
        if self.connection_options == *options {
            return Ok(());
        }

        let connection =
            Connection::open_with_flags(options.into_db_path(), OpenFlags::SQLITE_OPEN_READ_ONLY)?;

        self.entries = connection
            .prepare("SELECT institute, branch, quota, seatType, gender, orank, crank FROM data")?
            .query_map([], |row| {
                Ok(Entry {
                    institute: row.get(0)?,
                    branch: row.get(1)?,
                    quota: row.get(2)?,
                    seat_type: row.get(3)?,
                    gender: row.get(4)?,
                    or: row.get(5)?,
                    cr: row.get(6)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<Entry>>>()?;

        self.filters.load(&connection)?;
        self.connection = Some(connection);
        self.connection_options = *options;

        Ok(())
    }

    pub fn sort(&mut self, sort: &Sort) {
        self.entries.sort_by(match sort {
            Sort::OpeningAscending => |a: &Entry, b: &Entry| a.or.cmp(&b.or),
            Sort::OpeningDescending => |a: &Entry, b: &Entry| b.or.cmp(&a.or),
            Sort::ClosingAscending => |a: &Entry, b: &Entry| a.cr.cmp(&b.cr),
            Sort::ClosingDescending => |a: &Entry, b: &Entry| b.cr.cmp(&a.cr),
        });
    }

    pub fn get_filters(&mut self) -> &mut Filters {
        &mut self.filters
    }

    pub fn get_entries(&self) -> EntryIterator {
        EntryIterator::new(&self.filters, &self.entries)
    }
}
