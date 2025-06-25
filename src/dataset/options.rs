use std::{ops::RangeInclusive, path::PathBuf};

#[derive(Default)]
pub struct Options {
    pub year: Option<u16>,
    pub round: Option<u8>,
}

impl Options {
    pub fn get_valid_years() -> RangeInclusive<u16> {
        2016..=2024
    }

    pub fn get_valid_rounds(year: Option<u16>) -> RangeInclusive<u8> {
        match year {
            Some(year) => match year {
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
            },

            None => 4..=3, // Empty range
        }
    }

    pub fn is_complete(&self) -> bool {
        self.year.is_some() && self.round.is_some()
    }

    pub fn into_db_path(&self) -> PathBuf {
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
