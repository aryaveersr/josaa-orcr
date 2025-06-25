use crate::{Entry, Options};
use rusqlite::{Connection, OpenFlags};

pub struct Dataset {
    connection: Option<Connection>,
    entries: Vec<Entry>,
}

impl Dataset {
    pub fn new() -> Self {
        Self {
            connection: None,
            entries: Vec::new(),
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.connection.is_some()
    }

    pub fn get_entries(&self) -> &Vec<Entry> {
        &self.entries
    }

    pub fn load(&mut self, options: &Options) -> rusqlite::Result<()> {
        let connection =
            Connection::open_with_flags(options.into_db_path(), OpenFlags::SQLITE_OPEN_READ_ONLY)?;

        self.entries = connection
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

        self.connection = Some(connection);

        Ok(())
    }
}
