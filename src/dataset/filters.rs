use rusqlite::Connection;
use std::ops::RangeInclusive;

#[derive(Clone)]
pub struct Filters {
    pub institute: Vec<String>,
    pub quota: Vec<String>,
    pub seat_type: Vec<String>,
    pub gender: Vec<String>,
    pub or: RangeInclusive<u32>,
    pub cr: RangeInclusive<u32>,
}

impl Filters {
    fn get_institutes(connection: &Connection) -> rusqlite::Result<Vec<String>> {
        connection
            .prepare("SELECT DISTINCT institute FROM data;")?
            .query_map([], |row| Ok(row.get(0)?))?
            .collect::<rusqlite::Result<Vec<String>>>()
    }

    fn get_quotas(connection: &Connection) -> rusqlite::Result<Vec<String>> {
        connection
            .prepare("SELECT DISTINCT quota FROM data;")?
            .query_map([], |row| Ok(row.get(0)?))?
            .collect::<rusqlite::Result<Vec<String>>>()
    }

    fn get_seat_types(connection: &Connection) -> rusqlite::Result<Vec<String>> {
        connection
            .prepare("SELECT DISTINCT seatType FROM data;")?
            .query_map([], |row| Ok(row.get(0)?))?
            .collect::<rusqlite::Result<Vec<String>>>()
    }

    fn get_genders(connection: &Connection) -> rusqlite::Result<Vec<String>> {
        connection
            .prepare("SELECT DISTINCT gender FROM data;")?
            .query_map([], |row| Ok(row.get(0)?))?
            .collect::<rusqlite::Result<Vec<String>>>()
    }

    fn get_max_orank(connection: &Connection) -> rusqlite::Result<u32> {
        connection
            .prepare("SELECT MAX(orank) FROM data;")?
            .query_one([], |row| Ok(row.get(0)?))
    }

    fn get_max_crank(connection: &Connection) -> rusqlite::Result<u32> {
        connection
            .prepare("SELECT MAX(crank) FROM data;")?
            .query_one([], |row| Ok(row.get(0)?))
    }

    pub fn new(connection: &Connection) -> rusqlite::Result<Self> {
        Ok(Self {
            institute: Self::get_institutes(connection)?,
            quota: Self::get_quotas(connection)?,
            seat_type: Self::get_seat_types(connection)?,
            gender: Self::get_genders(connection)?,
            or: 0..=(Self::get_max_orank(connection))?,
            cr: 0..=(Self::get_max_crank(connection))?,
        })
    }
}
