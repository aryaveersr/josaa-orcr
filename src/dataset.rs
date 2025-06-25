use rusqlite::{Connection, OpenFlags};
use std::{ops::RangeInclusive, path::PathBuf};

pub const VALID_YEARS: RangeInclusive<u16> = 2016..=2024;

pub fn valid_rounds(year: u16) -> RangeInclusive<u8> {
    match year {
        2016 => 1..=6,
        2017 => 1..=7,
        2018 => 1..=7,
        2019 => 1..=7,
        2020 => 1..=6,
        2021 => 1..=6,
        2022 => 1..=6,
        2023 => 1..=6,
        2024 => 1..=5,

        _ => panic!("Invalid year: {year}."),
    }
}

#[derive(Default)]
pub struct DatasetOptions {
    pub year: Option<u16>,
    pub round: Option<u8>,
}

impl DatasetOptions {
    pub fn with_year(mut self, year: u16) -> Self {
        if !VALID_YEARS.contains(&year) {
            panic!("Invalid year: {year}.");
        }

        self.year = Some(year);
        self
    }

    pub fn with_round(mut self, round: u8) -> Self {
        let year = match self.year {
            Some(year) => year,
            None => panic!("No selected year while selecting a round."),
        };

        if !valid_rounds(year).contains(&round) {
            panic!("Invalid round: {round} for year: {year}.");
        }

        self.round = Some(round);
        self
    }

    fn get_db_path(&self) -> PathBuf {
        PathBuf::new()
            .join("db")
            .join(self.year.unwrap().to_string())
            .join(format!(
                "data-{}-{}.db",
                self.year.unwrap(),
                self.round.unwrap()
            ))
    }
}

pub struct Dataset {
    connection: Connection,

    pub rows: Vec<Row>,
}

impl Dataset {
    pub fn new(options: &DatasetOptions) -> rusqlite::Result<Self> {
        let connection =
            Connection::open_with_flags(options.get_db_path(), OpenFlags::SQLITE_OPEN_READ_ONLY)?;

        Ok(Self {
            connection,
            rows: Vec::new(),
        })
    }

    pub fn fetch_rows(&mut self) -> rusqlite::Result<()> {
        let mut stmt = self
            .connection
            .prepare("SELECT institute, quota, seatType, gender, orank, crank FROM data")?;

        let row_iter = stmt.query_map([], |row| {
            Ok(Row {
                institute: row.get(0)?,
                quota: row.get(1)?,
                seat_type: row.get(2)?,
                gender: row.get(3)?,
                or: row.get(4)?,
                cr: row.get(5)?,
            })
        })?;

        for row in row_iter {
            self.rows.push(row?);
        }

        Ok(())
    }
}

pub struct Row {
    pub institute: String,
    pub quota: String,
    pub seat_type: String,
    pub gender: String,
    pub or: u32,
    pub cr: u32,
}

impl Row {}
