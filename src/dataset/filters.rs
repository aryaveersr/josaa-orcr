use crate::{Entry, types::RankRange};
use rusqlite::Connection;
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct Filters {
    pub institute_kinds: HashMap<String, (bool, HashMap<String, bool>)>,
    pub branch: HashMap<String, bool>,
    pub quota: HashMap<String, bool>,
    pub seat_type: HashMap<String, bool>,
    pub gender: HashMap<String, bool>,
    pub or: RankRange,
    pub cr: RankRange,

    pub or_bounds: RankRange,
    pub cr_bounds: RankRange,
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
            .query_one([], |row| row.get(0))
    }

    fn get_institutes(&mut self, conn: &Connection) -> rusqlite::Result<()> {
        let kinds = conn
            .prepare("SELECT DISTINCT instituteType FROM institutes;")?
            .query_map([], |row| row.get(0))?
            .collect::<rusqlite::Result<Vec<String>>>()?;

        for kind in kinds {
            let institutes = conn
                .prepare("SELECT institute FROM institutes WHERE instituteType = ?1;")?
                .query_map([&kind], |row| Ok((row.get(0)?, true)))?
                .collect::<rusqlite::Result<HashMap<String, bool>>>()?;

            self.institute_kinds.insert(kind, (true, institutes));
        }

        Ok(())
    }

    pub fn load(&mut self, conn: &Connection) -> rusqlite::Result<()> {
        self.branch = Self::get_uniques(conn, "branch")?;
        self.quota = Self::get_uniques(conn, "quota")?;
        self.seat_type = Self::get_uniques(conn, "seatType")?;
        self.gender = Self::get_uniques(conn, "gender")?;

        self.or = (0..=(Self::get_max(conn, "orank"))?).into();
        self.cr = (0..=(Self::get_max(conn, "crank"))?).into();

        self.or_bounds = self.or;
        self.cr_bounds = self.cr;

        self.get_institutes(conn)?;

        Ok(())
    }

    pub fn matches(&self, entry: &Entry) -> bool {
        if !(*self.branch.get(&entry.branch).unwrap()
            && *self.quota.get(&entry.quota).unwrap()
            && *self.seat_type.get(&entry.seat_type).unwrap()
            && *self.gender.get(&entry.gender).unwrap()
            && self.or.contains(entry.or)
            && self.cr.contains(entry.cr))
        {
            return false;
        }

        let mut matches_institute = false;

        for (kind_enabled, institutes) in self.institute_kinds.values() {
            if let Some(institute_enabled) = institutes.get(&entry.institute) {
                matches_institute = *kind_enabled && *institute_enabled;
            }
        }

        matches_institute
    }
}
