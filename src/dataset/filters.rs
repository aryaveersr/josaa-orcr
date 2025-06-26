use rusqlite::Connection;
use std::{collections::HashMap, ops::RangeInclusive};

use crate::Entry;

#[derive(Clone)]
pub struct Filters {
    pub institute: HashMap<String, bool>,
    pub quota: HashMap<String, bool>,
    pub seat_type: HashMap<String, bool>,
    pub gender: HashMap<String, bool>,
    pub or: RangeInclusive<u32>,
    pub cr: RangeInclusive<u32>,

    pub or_bounds: RangeInclusive<u32>,
    pub cr_bounds: RangeInclusive<u32>,
}

impl Default for Filters {
    fn default() -> Self {
        Self {
            institute: HashMap::new(),
            quota: HashMap::new(),
            seat_type: HashMap::new(),
            gender: HashMap::new(),
            or: 4..=3,
            cr: 4..=3,
            or_bounds: 4..=3,
            cr_bounds: 4..=3,
        }
    }
}

impl Filters {
    fn get_uniques(
        conn: &Connection,
        field: &'static str,
    ) -> rusqlite::Result<HashMap<String, bool>> {
        conn.prepare(&format!("SELECT DISTINCT {field} FROM data;"))?
            .query_map([], |row| Ok((row.get(0)?, true)))?
            .collect()
    }

    fn get_max(conn: &Connection, field: &'static str) -> rusqlite::Result<u32> {
        conn.prepare(&format!("SELECT MAX({field}) FROM data;"))?
            .query_one([], |row| Ok(row.get(0)?))
    }

    pub fn load(&mut self, conn: &Connection) -> rusqlite::Result<()> {
        self.institute = Self::get_uniques(conn, "institute")?;
        self.quota = Self::get_uniques(conn, "quota")?;
        self.seat_type = Self::get_uniques(conn, "seatType")?;
        self.gender = Self::get_uniques(conn, "gender")?;

        self.or = 0..=(Self::get_max(conn, "orank"))?;
        self.cr = 0..=(Self::get_max(conn, "crank"))?;

        self.or_bounds = self.or.clone();
        self.cr_bounds = self.cr.clone();

        Ok(())
    }

    pub fn matches(&self, entry: &Entry) -> bool {
        *self.institute.get(&entry.institute).unwrap()
            && *self.quota.get(&entry.quota).unwrap()
            && *self.seat_type.get(&entry.seat_type).unwrap()
            && *self.gender.get(&entry.gender).unwrap()
            && self.or.contains(&entry.or)
            && self.cr.contains(&entry.cr)
    }
}
