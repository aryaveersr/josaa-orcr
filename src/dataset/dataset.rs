use crate::{Entry, Filters, Options, Sort};
use rusqlite::{Connection, OpenFlags};

pub struct Dataset {
    connection: Option<Connection>,
    connection_options: Options,

    entries: Vec<Entry>,
}

impl Dataset {
    pub fn new() -> Self {
        Self {
            connection: None,
            connection_options: Options::default(),

            entries: Vec::new(),
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.connection.is_some()
    }

    pub fn get_entries(&self) -> &Vec<Entry> {
        &self.entries
    }

    pub fn get_connection(&self) -> &Connection {
        self.connection.as_ref().unwrap()
    }

    pub fn create_connection(&mut self, options: &Options) -> rusqlite::Result<()> {
        if self.connection_options == *options {
            return Ok(());
        }

        self.connection = Some(Connection::open_with_flags(
            options.into_db_path(),
            OpenFlags::SQLITE_OPEN_READ_ONLY,
        )?);

        self.connection_options = *options;

        Ok(())
    }

    pub fn load(&mut self, filters: &Filters, sort: Sort) -> rusqlite::Result<()> {
        self.entries = self
            .connection
            .as_ref()
            .unwrap()
            .prepare("SELECT institute, quota, seatType, gender, orank, crank FROM data")?
            .query_map([], |row| {
                Ok(Entry {
                    institute: row.get(0)?,
                    quota: row.get(1)?,
                    seat_type: row.get(2)?,
                    gender: row.get(3)?,
                    or: row.get(4)?,
                    cr: row.get(5)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<Entry>>>()?;

        self.entries.retain(|entry| {
            filters.institute.contains(&entry.institute)
                && filters.quota.contains(&entry.quota)
                && filters.seat_type.contains(&entry.seat_type)
                && filters.gender.contains(&entry.gender)
                && filters.or.contains(&entry.or)
                && filters.cr.contains(&entry.cr)
        });

        self.entries.sort_by(match sort {
            Sort::OpeningAscending => |a: &Entry, b: &Entry| a.or.cmp(&b.or),
            Sort::OpeningDescending => |a: &Entry, b: &Entry| b.or.cmp(&a.or),
            Sort::ClosingAscending => |a: &Entry, b: &Entry| a.cr.cmp(&b.cr),
            Sort::ClosingDescending => |a: &Entry, b: &Entry| b.cr.cmp(&a.cr),
        });

        Ok(())
    }
}
